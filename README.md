# Pyrin-miner
[![Build status](https://github.com/tmrlvi/pyrin-miner/workflows/ci/badge.svg)](https://github.com/tmrlvi/pyrin-miner/actions)
[![Latest version](https://img.shields.io/crates/v/pyrin-miner.svg)](https://crates.io/crates/pyrin-miner)
![License](https://img.shields.io/crates/l/pyrin-miner.svg)
[![dependency status](https://deps.rs/repo/github/tmrlvi/pyrin-miner/status.svg)](https://deps.rs/repo/github/tmrlvi/pyrin-miner)

[![Discord](https://discordapp.com/api/guilds/599153230659846165/embed.png)](https://discord.gg/kS3SK5F36R)
[![Telegram](https://img.shields.io/badge/Telegram-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white)](https://t.me/pyrinenglish)


## Installation
### From Sources
Installing via `cargo install` is not supported for the latest version.

The regular version is still available at
```sh
cargo install pyrin-miner
```

### From Git Sources

If you are looking to build from the repository (for debug / extension), note that the plugins are additional
packages in the workspace. To compile a specific package, you run the following command or any subset of it

```sh
git clone git@github.com:tmrlvi/pyrin-miner.git
cd pyrin-miner
cargo build --release -p pyrin-miner -p pyrincuda -p pyrinopencl
```
And, the miner (and plugins) will be in `targets/release`. You can replace the last line with
```sh
cargo build --release --all
```

### From Binaries
The [release page](https://github.com/tmrlvi/pyrin-miner/releases) includes precompiled binaries for Linux, and Windows (for the GPU version).

### Removing Plugins
To remove a plugin, you simply remove the corresponding `dll`/`so` for the directory of the miner. 

* `libpyrincuda.so`, `libpyrincuda.dll`: Cuda support for Pyrin-Miner
* `libpyrinopencl.so`, `libpyrinopencl.dll`: OpenCL support for Pyrin-Miner

# Usage
To start mining, you need to run [pyrin](https://github.com/Pyrinpyi/pyipad) and have an address to send the rewards to.
Here is a guidance on how to run a full node and how to generate addresses: https://github.com/pyrinnet/docs/blob/main/Getting%20Started/Full%20Node%20Installation.md

Help:
```
pyrin-miner 
A Pyrin high performance CPU miner

USAGE:
    pyrin-miner [OPTIONS] --mining-address <MINING_ADDRESS>

OPTIONS:
    -a, --mining-address <MINING_ADDRESS>                  The Pyrin address for the miner reward
        --cuda-device <CUDA_DEVICE>                        Which CUDA GPUs to use [default: all]
        --cuda-disable                                     Disable cuda workers
        --cuda-lock-core-clocks <CUDA_LOCK_CORE_CLOCKS>    Lock core clocks eg: ,1200, [default: 0]
        --cuda-lock-mem-clocks <CUDA_LOCK_MEM_CLOCKS>      Lock mem clocks eg: ,810, [default: 0]
        --cuda-no-blocking-sync                            Actively wait for result. Higher CPU usage, but less red blocks. Can have lower workload.
        --cuda-power-limits <CUDA_POWER_LIMITS>            Lock power limits eg: ,150, [default: 0]
        --cuda-workload <CUDA_WORKLOAD>                    Ratio of nonces to GPU possible parrallel run [default: 64]
        --cuda-workload-absolute                           The values given by workload are not ratio, but absolute number of nonces [default: false]
    -d, --debug                                            Enable debug logging level
        --experimental-amd                                 Uses SMID instructions in AMD. Miner will crash if instruction is not supported
    -h, --help                                             Print help information
        --mine-when-not-synced                             Mine even when pyrin says it is not synced
        --nonce-gen <NONCE_GEN>                            The random method used to generate nonces. Options: (i) xoshiro (ii) lean [default: lean]
        --opencl-amd-disable                               Disables AMD mining (does not override opencl-enable)
        --opencl-device <OPENCL_DEVICE>                    Which OpenCL GPUs to use on a specific platform
        --opencl-enable                                    Enable opencl, and take all devices of the chosen platform
        --opencl-no-amd-binary                             Disable fetching of precompiled AMD kernel (if exists)
        --opencl-platform <OPENCL_PLATFORM>                Which OpenCL platform to use (limited to one per executable)
        --opencl-workload <OPENCL_WORKLOAD>                Ratio of nonces to GPU possible parrallel run in OpenCL [default: 512]
        --opencl-workload-absolute                         The values given by workload are not ratio, but absolute number of nonces in OpenCL [default: false]
    -p, --port <PORT>                                      Pyipad port [default: Mainnet = 13110, Testnet = 16211]
    -s, --pyrin-address <pyrin_ADDRESS>                  The IP of the pyrin instance [default: 127.0.0.1]
    -t, --threads <NUM_THREADS>                            Amount of CPU miner threads to launch [default: 0]
        --testnet                                          Use testnet instead of mainnet [default: false]
```

To start mining, you just need to run the following:

`./pyrin-miner --mining-address pyrin:XXXXX`

This will run the miner on all the available GPU devcies.
