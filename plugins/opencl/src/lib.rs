#[macro_use]
extern crate kaspa_miner;

use clap::{ArgMatches, FromArgMatches};
use kaspa_miner::{Plugin, Worker, WorkerSpec};
use log::{info, LevelFilter};
use opencl3::device::{Device, CL_DEVICE_TYPE_ALL};
use opencl3::platform::{get_platforms, Platform};
use opencl3::types::cl_device_id;
use std::error::Error as StdError;

pub type Error = Box<dyn StdError + Send + Sync + 'static>;

mod cli;
mod worker;

use crate::cli::{NonceGenEnum, OpenCLOpt};
use crate::worker::OpenCLGPUWorker;

const DEFAULT_WORKLOAD_SCALE: f32 = 512.;

pub struct OpenCLPlugin {
    specs: Vec<OpenCLWorkerSpec>,
    _enabled: bool,
}

impl OpenCLPlugin {
    fn new() -> Result<Self, Error> {
        env_logger::builder().filter_level(LevelFilter::Info).parse_default_env().init();
        Ok(Self { specs: Vec::new(), _enabled: false })
    }
}

impl Plugin for OpenCLPlugin {
    fn name(&self) -> &'static str {
        "OpenCL Worker"
    }

    fn enabled(&self) -> bool {
        self._enabled
    }

    fn get_worker_specs(&self) -> Vec<Box<dyn WorkerSpec>> {
        self.specs.iter().map(|spec| Box::new(*spec) as Box<dyn WorkerSpec>).collect::<Vec<Box<dyn WorkerSpec>>>()
    }

    //noinspection RsTypeCheck
    fn process_option(&mut self, matches: &ArgMatches) -> Result<usize, kaspa_miner::Error> {
        let opts: OpenCLOpt = OpenCLOpt::from_arg_matches(matches)?;

        self._enabled = opts.opencl_enable;
        let platforms = match get_platforms() {
            Ok(p) => p,
            Err(e) => {
                return Err(e.to_string().into());
            }
        };
        info!("OpenCL Found Platforms:");
        info!("=======================");
        for platform in &platforms {
            let vendor = &platform.vendor().unwrap_or_else(|_| "Unk".into());
            let name = &platform.name().unwrap_or_else(|_| "Unk".into());
            let num_devices = platform.get_devices(CL_DEVICE_TYPE_ALL).unwrap_or_default().len();
            info!("{}: {} ({} devices available)", vendor, name, num_devices);
        }
        let amd_platforms = (&platforms)
            .iter()
            .filter(|p| {
                p.vendor().unwrap_or_else(|_| "Unk".into()) == "Advanced Micro Devices, Inc."
                    && !p.get_devices(CL_DEVICE_TYPE_ALL).unwrap_or_default().is_empty()
            })
            .collect::<Vec<&Platform>>();
        let _platform: &Platform = match opts.opencl_platform {
            Some(idx) => {
                self._enabled = true;
                &platforms[idx as usize]
            }
            None if !opts.opencl_amd_disable && !amd_platforms.is_empty() => {
                self._enabled = true;
                amd_platforms[0]
            }
            None => &platforms[0],
        };
        if self._enabled {
            info!(
                "Chose to mine on {}: {}.",
                &_platform.vendor().unwrap_or_else(|_| "Unk".into()),
                &_platform.name().unwrap_or_else(|_| "Unk".into())
            );

            let device_ids = _platform.get_devices(CL_DEVICE_TYPE_ALL).unwrap();
            let gpus = match opts.opencl_device {
                Some(dev) => {
                    self._enabled = true;
                    dev.iter().map(|d| device_ids[*d as usize]).collect::<Vec<cl_device_id>>()
                }
                None => device_ids,
            };

            self.specs = (0..gpus.len())
                .map(|i| OpenCLWorkerSpec {
                    _platform: *_platform,
                    index: i,
                    device_id: Device::new(gpus[i]),
                    workload: match &opts.opencl_workload {
                        Some(workload) if i < workload.len() => workload[i],
                        Some(workload) if !workload.is_empty() => *workload.last().unwrap(),
                        _ => DEFAULT_WORKLOAD_SCALE,
                    },
                    is_absolute: opts.opencl_workload_absolute,
                    experimental_amd: opts.experimental_amd,
                    use_amd_binary: !opts.opencl_no_amd_binary,
                    random: opts.opencl_nonce_gen,
                })
                .collect();
        }
        Ok(self.specs.len())
    }
}

#[derive(Copy, Clone)]
struct OpenCLWorkerSpec {
    _platform: Platform,
    index: usize,
    device_id: Device,
    workload: f32,
    is_absolute: bool,
    experimental_amd: bool,
    use_amd_binary: bool,
    random: NonceGenEnum,
}

impl WorkerSpec for OpenCLWorkerSpec {
    fn id(&self) -> String {
        format!(
            "#{} {}",
            self.index,
            self.device_id
                .board_name_amd()
                .unwrap_or_else(|_| self.device_id.name().unwrap_or_else(|_| "Unknown Device".into()))
        )
    }

    fn build(&self) -> Box<dyn Worker> {
        Box::new(
            OpenCLGPUWorker::new(
                self.device_id,
                self.workload,
                self.is_absolute,
                self.experimental_amd,
                self.use_amd_binary,
                &self.random,
            )
            .unwrap(),
        )
    }
}

declare_plugin!(OpenCLPlugin, OpenCLPlugin::new, OpenCLOpt);
