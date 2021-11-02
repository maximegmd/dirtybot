use crate::{config::Config, errors::DirtyError};

pub struct Blockchain {
    web3: web3::Web3<web3::transports::Http>
}

impl Blockchain {
    pub fn new(config: Config) -> Result<Self, DirtyError> {
        let result = web3::transports::Http::new(&config.rpc_endpoint)?;

        Ok(Self {
            web3: web3::Web3::new(result)
        })
    }
}