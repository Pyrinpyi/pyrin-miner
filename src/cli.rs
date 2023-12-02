use clap::Parser;
use log::LevelFilter;

use crate::Error;

#[derive(Parser, Debug)]
#[clap(name = "pyrin-miner", version, about = "A Pyrin high performance CPU miner", term_width = 0)]
pub struct Opt {
    #[clap(short, long, help = "Enable debug logging level")]
    pub debug: bool,
    #[clap(short = 'a', long = "mining-address", help = "The Pyrin address for the miner reward")]
    pub mining_address: String,
    #[clap(short = 's', long = "pyrin-address", default_value = "127.0.0.1", help = "The IP of the pyrin instance")]
    pub pyrin_address: String,

    #[clap(short, long, help = "Pyipad port [default: Mainnet = 16110, Testnet = 16211]")]
    port: Option<u16>,

    #[clap(long, help = "Use testnet instead of mainnet [default: false]")]
    testnet: bool,
    #[clap(short = 't', long = "threads", help = "Amount of CPU miner threads to launch [default: 0]")]
    pub num_threads: Option<u16>,
    #[clap(
        long = "mine-when-not-synced",
        help = "Mine even when pyrin says it is not synced",
        long_help = "Mine even when pyrin says it is not synced, only useful when passing `--allow-submit-block-when-not-synced` to pyrin  [default: false]"
    )]
    pub mine_when_not_synced: bool,

    #[clap(skip)]
    pub devfund_address: String,
}

impl Opt {
    pub fn process(&mut self) -> Result<(), Error> {
        //self.gpus = None;
        if self.pyrin_address.is_empty() {
            self.pyrin_address = "127.0.0.1".to_string();
        }

        if !self.pyrin_address.contains("://") {
            let port_str = self.port().to_string();
            let (pyrin, port) = match self.pyrin_address.contains(':') {
                true => self.pyrin_address.split_once(':').expect("We checked for `:`"),
                false => (self.pyrin_address.as_str(), port_str.as_str()),
            };
            self.pyrin_address = format!("grpc://{}:{}", pyrin, port);
        }
        log::info!("pyrin address: {}", self.pyrin_address);

        if self.num_threads.is_none() {
            self.num_threads = Some(0);
        }

        let miner_network = self.mining_address.split(':').next();
        self.devfund_address = String::from("pyrin:qzj9kz0kmc3rxl9mw86mlda2cqmvp3xhavx9h2jud5ehdchvruql6ey64r8kz");
        let devfund_network = self.devfund_address.split(':').next();
        if miner_network.is_some() && devfund_network.is_some() && miner_network != devfund_network {
            log::info!(
                "Mining address ({}) and devfund ({}) are not from the same network. Disabling devfund.",
                miner_network.unwrap(),
                devfund_network.unwrap()
            )
        }
        Ok(())
    }

    fn port(&mut self) -> u16 {
        *self.port.get_or_insert(if self.testnet { 16211 } else { 16110 })
    }

    pub fn log_level(&self) -> LevelFilter {
        if self.debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        }
    }
}
