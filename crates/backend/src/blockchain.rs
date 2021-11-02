use std::str::FromStr;

use log::info;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use web3::types::{Address, H160, TransactionParameters, TransactionRequest, U256};
use tiny_hderive::{bip32::ExtendedPrivKey};
use bip39::{Mnemonic, Language};

use crate::{config::Config, errors::DirtyError};

#[derive(Clone)]
pub struct Blockchain {
    web3: web3::Web3<web3::transports::Http>,
    basekey: ExtendedPrivKey
}

impl Blockchain {
    pub fn new(config: &Config) -> Result<Self, DirtyError> {
        let result = web3::transports::Http::new(&config.rpc_endpoint)?;

        info!("Blockchain transport bound to {}", &config.rpc_endpoint);

        let mnemonic = Mnemonic::parse_in(Language::English, &config.passphrase).map_err(|_| DirtyError::InvalidPassphrase)?;

        let seed =  mnemonic.to_entropy_array().0;

        let ext = ExtendedPrivKey::derive(&seed, "m/44'/60'/0'/0/0").unwrap();

        Ok(Self {
            web3: web3::Web3::new(result),
            basekey: ext
        })
    }

    pub async fn get_balance(&self, address: String) -> Result<U256, DirtyError> {
        let addr = if let Ok(addr) = H160::from_str(&address) {
            addr
        } else {
            return Err(DirtyError::InvalidAddress)
        };

        let balance = self.web3.eth().balance(addr, None).await?;

        Ok(balance)
    }

    pub async fn send(&self, address: String, value: U256) -> Result<U256, DirtyError> {

        let addr = if let Ok(addr) = H160::from_str(&address) {
            addr
        } else {
            return Err(DirtyError::InvalidAddress)
        };

        let accounts = self.web3.eth().accounts().await?;

        let _balance_before = self.web3.eth().balance(accounts[0], None).await?;

        let tx = TransactionRequest {
            from: accounts[0],
            to: Some(addr),
            gas: None,
            gas_price: None,
            value: Some(value),
            data: None,
            nonce: None,
            condition: None,
            ..Default::default()
        };

        let _tx_hash = self.web3.eth().send_transaction(tx).await?;

        let balance_after = self.web3.eth().balance(addr, None).await?;

        info!("Sent {} Gwei from {} to {}", value, accounts[0], addr);

        Ok(balance_after)
    }

    

    pub async fn deposit(&mut self) -> Result<String, DirtyError> {

        let pvk = web3::types::H256::from_slice(&self.basekey.secret());
        let pubk = web3::types::H160::from(pvk);

        let tx_object = TransactionParameters {
            to: Some(Address::from_str("0x8E35bf056f028dd4d0b94ec5616D53Da0013C516").unwrap()),
            value: U256::exp10(16), //0.01 eth
            chain_id: Some(46834),
            nonce: Some(U256::from(0)),
            gas: U256::from(21000),
            gas_price: Some(U256::from(1000000000)),
            ..Default::default()
        };

        let secp = Secp256k1::new();

        let pk = SecretKey::from_slice(&self.basekey.secret()).unwrap();
        let pubkk = PublicKey::from_secret_key(&secp, &pk);

        let public_addr = &web3::signing::keccak256(&pubkk.serialize_uncompressed()[1..])[12..];

        print!("0x");
        for c in public_addr {
            print!("{:x}", c);
        }

        // Sign the tx (can be done offline)
        let signed = self.web3.accounts().sign_transaction(tx_object, &pk).await?;
        info!("{:?}", signed);
        let result = self.web3.eth().send_raw_transaction(signed.raw_transaction).await?;

        Ok("".into())
    }
}