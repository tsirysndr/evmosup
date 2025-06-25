use std::fs;

use anyhow::Error;

use crate::types::EvmosUpConfig;

pub fn init(nb_accounts: usize) -> Result<(), Error> {
    let mut config = EvmosUpConfig::new();
    config.generate_accounts(nb_accounts)?;
    let config_toml = toml::to_string(&config)
        .map_err(|e| Error::msg(format!("Failed to serialize config: {}", e)))?;
    fs::write("evmosup.toml", config_toml)?;
    Ok(())
}
