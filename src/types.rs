use std::process::Command;

use anyhow::Error;
use serde::{Deserialize, Serialize};

pub struct AppConfig {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub name: String,
    pub mnemonic: String,
}

#[derive(Serialize, Deserialize)]
pub struct EvmosUpConfig {
    pub chain_id: String,
    pub keyring_backend: String,
    pub home: String,
    pub key_algo: String,
    pub base_denom: String,
    pub moniker: String,
    pub basefee: u64,
    pub minimum_gas_prices: u64,
    pub genesis_accounts: Vec<Account>,
}

impl Default for EvmosUpConfig {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap();
        let home: String = home.join(".evmosd").display().to_string();
        EvmosUpConfig {
            chain_id: "evmos_9001-2".to_string(),
            keyring_backend: "test".to_string(),
            home,
            key_algo: "eth_secp256k1".to_string(),
            base_denom: "aevmos".to_string(),
            moniker: "localtestnet".to_string(),
            basefee: 1000000,
            minimum_gas_prices: 1000000,
            genesis_accounts: vec![
              Account {
                name: "val_key".to_string(),
                mnemonic: "gesture inject test cycle original hollow east ridge hen combine junk child bacon zero hope comfort vacuum milk pitch cage oppose unhappy lunar seat".to_string(),
              },
              Account {
                name: "user1_key".to_string(),
                mnemonic: "copper push brief egg scan entry inform record adjust fossil boss egg comic alien upon aspect dry avoid interest fury window hint race symptom".to_string(),
              },
              Account {
                name: "user2_key".to_string(),
                mnemonic: "maximum display century economy unlock van census kite error heart snow filter midnight usage egg venture cash kick motor survey drastic edge muffin visual".to_string(),
              },
              Account {
                name: "user3_key".to_string(),
                mnemonic: "will wear settle write dance topic tape sea glory hotel oppose rebel client problem era video gossip glide during yard balance cancel file rose".to_string(),
              },
              Account {
                name: "user4_key".to_string(),
                mnemonic: "doll midnight silk carpet brush boring pluck office gown inquiry duck chief aim exit gain never tennis crime fragile ship cloud surface exotic patch".to_string(),
              },
            ],
        }
    }
}

impl EvmosUpConfig {
    pub fn new() -> Self {
        EvmosUpConfig {
            genesis_accounts: vec![],
            ..EvmosUpConfig::default()
        }
    }

    pub fn generate_accounts(&mut self, num_accounts: usize) -> Result<(), Error> {
        self.genesis_accounts.clear();
        for i in 0..num_accounts {
            let cmd = Command::new("evmosd")
                .arg("keys")
                .arg("mnemonic")
                .output()?;
            if !cmd.status.success() {
                return Err(Error::msg("Failed to generate mnemonic for account"));
            }
            let mnemonic = String::from_utf8(cmd.stdout)?.trim().to_string();
            let account = Account {
                name: format!("user{}_key", i + 1),
                mnemonic,
            };
            self.genesis_accounts.push(account);
        }
        Ok(())
    }
}
