use crate::{Error, NonceGenEnum};
use cust::context::CurrentContext;
use cust::device::DeviceAttribute;
use cust::function::Function;
use cust::module::{ModuleJitOption, OptLevel};
use cust::prelude::*;
use pyrin_miner::xoshiro256starstar::Xoshiro256StarStar;
use pyrin_miner::Worker;
use log::{error, info};
use rand::{Fill, RngCore};
use std::ffi::CString;
use std::sync::{Arc, Weak};

static BPS: f32 = 1.;

static PTX_86: &str = include_str!("../resources/pyrin-cuda-sm86.ptx");
static PTX_75: &str = include_str!("../resources/pyrin-cuda-sm75.ptx");
static PTX_61: &str = include_str!("../resources/pyrin-cuda-sm61.ptx");
static PTX_30: &str = include_str!("../resources/pyrin-cuda-sm30.ptx");
static PTX_20: &str = include_str!("../resources/pyrin-cuda-sm20.ptx");

pub struct Kernel<'kernel> {
    func: Arc<Function<'kernel>>,
    block_size: u32,
    grid_size: u32,
}

impl<'kernel> Kernel<'kernel> {
    pub fn new(module: Weak<Module>, name: &'kernel str) -> Result<Kernel<'kernel>, Error> {
        let func = Arc::new(unsafe {
            module.as_ptr().as_ref().unwrap().get_function(name).map_err(|e| {
                error!("Error loading function: {}", e);
                e
            })?
        });
        let (_, block_size) = func.suggested_launch_configuration(0, 0.into())?;

        let device = CurrentContext::get_device()?;
        let sm_count = device.get_attribute(DeviceAttribute::MultiprocessorCount)? as u32;
        let grid_size = sm_count * func.max_active_blocks_per_multiprocessor(block_size.into(), 0)?;

        Ok(Self { func, block_size, grid_size })
    }

    pub fn get_workload(&self) -> u32 {
        self.block_size * self.grid_size
    }

    pub fn set_workload(&mut self, workload: u32) {
        self.grid_size = (workload + self.block_size - 1) / self.block_size
    }
}

pub struct CudaGPUWorker<'gpu> {
    // NOTE: The order is important! context must be closed last
    heavy_hash_kernel: Kernel<'gpu>,
    stream: Stream,
    start_event: Event,
    stop_event: Event,
    _module: Arc<Module>,

    rand_state: DeviceBuffer<u64>,
    final_nonce_buff: DeviceBuffer<u64>,

    device_id: u32,
    pub workload: usize,
    _context: Context,

    random: NonceGenEnum,
}

impl<'gpu> Worker for CudaGPUWorker<'gpu> {
    fn id(&self) -> String {
        let device = CurrentContext::get_device().unwrap();
        format!("#{} ({})", self.device_id, device.name().unwrap())
    }

    fn load_block_constants(&mut self, hash_header: &[u8; 72], matrix: &[[u16; 64]; 64], target: &[u64; 4]) {
        let u8matrix: Arc<[[u8; 64]; 64]> = Arc::new(matrix.map(|row| row.map(|v| v as u8)));
        let mut hash_header_gpu = self._module.get_global::<[u8; 72]>(&CString::new("hash_header").unwrap()).unwrap();
        hash_header_gpu.copy_from(hash_header).map_err(|e| e.to_string()).unwrap();

        let mut matrix_gpu = self._module.get_global::<[[u8; 64]; 64]>(&CString::new("matrix").unwrap()).unwrap();
        matrix_gpu.copy_from(&u8matrix).map_err(|e| e.to_string()).unwrap();

        let mut target_gpu = self._module.get_global::<[u64; 4]>(&CString::new("target").unwrap()).unwrap();
        target_gpu.copy_from(target).map_err(|e| e.to_string()).unwrap();
    }

    #[inline(always)]
    fn calculate_hash(&mut self, _nonces: Option<&Vec<u64>>, nonce_mask: u64, nonce_fixed: u64) {
        let func = &self.heavy_hash_kernel.func;
        let stream = &self.stream;
        let random: u8 = match self.random {
            NonceGenEnum::Lean => {
                self.rand_state.copy_from(&[rand::thread_rng().next_u64()]).unwrap();
                0
            }
            NonceGenEnum::Xoshiro => 1,
        };

        self.start_event.record(stream).unwrap();
        unsafe {
            launch!(
                func<<<
                    self.heavy_hash_kernel.grid_size, self.heavy_hash_kernel.block_size,
                    0, stream
                >>>(
                    nonce_mask, nonce_fixed,
                    self.workload,
                    random,
                    self.rand_state.as_device_ptr(),
                    self.final_nonce_buff.as_device_ptr()
                )
            )
            .unwrap(); // We see errors in sync
        }
        self.stop_event.record(stream).unwrap();
    }

    #[inline(always)]
    fn sync(&self) -> Result<(), Error> {
        //self.stream.synchronize()?;
        self.stop_event.synchronize()?;
        if self.stop_event.elapsed_time_f32(&self.start_event)? > 1000. / BPS {
            return Err("Cuda takes longer then block rate. Please reduce your workload.".into());
        }
        Ok(())
    }

    fn get_workload(&self) -> usize {
        self.workload
    }

    #[inline(always)]
    fn copy_output_to(&mut self, nonces: &mut Vec<u64>) -> Result<(), Error> {
        self.final_nonce_buff.copy_to(nonces)?;
        Ok(())
    }
}

impl<'gpu> CudaGPUWorker<'gpu> {
    pub fn new(
        device_id: u32,
        workload: f32,
        is_absolute: bool,
        blocking_sync: bool,
        random: NonceGenEnum,
    ) -> Result<Self, Error> {
        info!("Starting a CUDA worker");
        let sync_flag = match blocking_sync {
            true => ContextFlags::SCHED_BLOCKING_SYNC,
            false => ContextFlags::SCHED_AUTO,
        };
        let device = Device::get_device(device_id).unwrap();
        let _context = Context::new(device)?;
        _context.set_flags(sync_flag)?;

        let major = device.get_attribute(DeviceAttribute::ComputeCapabilityMajor)?;
        let minor = device.get_attribute(DeviceAttribute::ComputeCapabilityMinor)?;
        let _module: Arc<Module>;
        info!("Device #{} compute version is {}.{}", device_id, major, minor);
        if major > 8 || (major == 8 && minor >= 6) {
            _module = Arc::new(Module::from_ptx(PTX_86, &[ModuleJitOption::OptLevel(OptLevel::O4)]).map_err(|e| {
                error!("Error loading PTX. Make sure you have the updated driver for you devices");
                e
            })?);
        } else if major > 7 || (major == 7 && minor >= 5) {
            _module = Arc::new(Module::from_ptx(PTX_75, &[ModuleJitOption::OptLevel(OptLevel::O4)]).map_err(|e| {
                error!("Error loading PTX. Make sure you have the updated driver for you devices");
                e
            })?);
        } else if major > 6 || (major == 6 && minor >= 1) {
            _module = Arc::new(Module::from_ptx(PTX_61, &[ModuleJitOption::OptLevel(OptLevel::O4)]).map_err(|e| {
                error!("Error loading PTX. Make sure you have the updated driver for you devices");
                e
            })?);
        }/* else if major >= 3 {
            _module = Arc::new(Module::from_ptx(PTX_30, &[ModuleJitOption::OptLevel(OptLevel::O4)]).map_err(|e| {
                error!("Error loading PTX. Make sure you have the updated driver for you devices");
                e
            })?);
        } else if major >= 2 {
            _module = Arc::new(Module::from_ptx(PTX_20, &[ModuleJitOption::OptLevel(OptLevel::O4)]).map_err(|e| {
                error!("Error loading PTX. Make sure you have the updated driver for you devices");
                e
            })?);
        }*/ else {
            return Err("Cuda compute version not supported".into());
        }

        let stream = Stream::new(StreamFlags::NON_BLOCKING, None)?;

        let mut heavy_hash_kernel = Kernel::new(Arc::downgrade(&_module), "heavy_hash")?;

        let mut chosen_workload = 0u32;
        if is_absolute {
            chosen_workload = 1;
        } else {
            let cur_workload = heavy_hash_kernel.get_workload();
            if chosen_workload == 0 || chosen_workload < cur_workload {
                chosen_workload = cur_workload;
            }
        }
        chosen_workload = (chosen_workload as f32 * workload) as u32;
        info!("GPU #{} Chosen workload: {}", device_id, chosen_workload);
        heavy_hash_kernel.set_workload(chosen_workload);

        let final_nonce_buff = vec![0u64; 1].as_slice().as_dbuf()?;

        let rand_state: DeviceBuffer<u64> = match random {
            NonceGenEnum::Xoshiro => {
                info!("Using xoshiro for nonce-generation");
                let mut buffer = DeviceBuffer::<u64>::zeroed(4 * (chosen_workload as usize)).unwrap();
                info!("GPU #{} is generating initial seed. This may take some time.", device_id);
                let mut seed = [1u64; 4];
                seed.try_fill(&mut rand::thread_rng())?;
                buffer.copy_from(
                    Xoshiro256StarStar::new(&seed)
                        .iter_jump_state()
                        .take(chosen_workload as usize)
                        .flatten()
                        .collect::<Vec<u64>>()
                        .as_slice(),
                )?;
                info!("GPU #{} initialized", device_id);
                buffer
            }
            NonceGenEnum::Lean => {
                info!("Using lean nonce-generation");
                let mut buffer = DeviceBuffer::<u64>::zeroed(1).unwrap();
                let seed = rand::thread_rng().next_u64();
                buffer.copy_from(&[seed])?;
                buffer
            }
        };
        Ok(Self {
            device_id,
            _context,
            _module,
            start_event: Event::new(EventFlags::DEFAULT)?,
            stop_event: Event::new(EventFlags::DEFAULT)?,
            workload: chosen_workload as usize,
            stream,
            rand_state,
            final_nonce_buff,
            heavy_hash_kernel,
            random,
        })
    }
}
