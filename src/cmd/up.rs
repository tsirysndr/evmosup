use anyhow::Error;
use owo_colors::OwoColorize;
use std::{path::Path, process::Command};

use crate::types::EvmosUpConfig;

pub fn up() -> Result<(), Error> {
    if verify_evmosd_home().is_ok() {
        start()?;
        return Ok(());
    }

    set_config()?;
    import_keys()?;
    set_gas_limit()?;
    set_base_fee()?;
    init()?;
    enable_api()?;
    disable_rosetta_api()?;
    disable_memiavl()?;
    disable_versiondb()?;
    change_proposal_periods()?;
    allocate_genesis_accounts()?;
    sign_genesis_transaction()?;
    collect_gentxs()?;
    validate_genesis()?;

    println!("{}", "Evmos genesis setup completed successfully.".cyan());

    start()?;

    Ok(())
}

fn verify_evmosd_home() -> Result<(), Error> {
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");

    if evmos_home.is_dir() {
        return Ok(());
    }

    return Err(Error::msg(format!(
        "Evmos home directory not found at {}. Please run `evmosd init` first.",
        evmos_home.display()
    )));
}

fn set_config() -> Result<(), Error> {
    let config = load_config()?;

    Command::new("sh")
        .arg("-c")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&format!(
            "evmosd config set client chain-id {} --home {} && evmosd config set client keyring-backend {} --home {}",
            config.chain_id,
            config.home,
            config.keyring_backend,
            config.home
        ))
        .spawn()?
        .wait()?;

    Ok(())
}

fn import_keys() -> Result<(), Error> {
    let config = load_config()?;
    for account in config.genesis_accounts.iter() {
        if config.keyring_backend != "test" {
            Command::new("sh")
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .arg("-c")
                .arg(format!(
                    "evmosd keys add '{}' --recover --keyring-backend {} --algo {} --home {}",
                    account.name, config.keyring_backend, config.key_algo, config.home
                ))
                .spawn()?
                .wait()?;
            continue;
        }
        Command::new("sh")
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .arg("-c")
            .arg(format!(
                "echo '{}' | evmosd keys add '{}' --recover --keyring-backend {} --algo {} --home {}",
                account.mnemonic,
                account.name,
                config.keyring_backend,
                config.key_algo,
                config.home
            ))
            .spawn()?
            .wait()?;
    }

    Ok(())
}

fn set_gas_limit() -> Result<(), Error> {
    let config = load_config()?;
    Command::new("sh")
        .arg("-c")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&format!(r#"jq '.consensus_params["block"]["max_gas"]="10000000"' {}/config/genesis.json >/tmp/genesis.json && mv /tmp/genesis.json {}/config/genesis.json"#, config.home, config.home))
        .spawn()?
        .wait()?;
    Ok(())
}

fn set_base_fee() -> Result<(), Error> {
    let config = load_config()?;
    Command::new("sh")
        .arg("-c")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&format!(r#"jq '.app_state["feemarket"]["params"]["base_fee"]='{}' {}/config/genesis.json >/tmp/genesis.json && mv /tmp/genesis.json {}/config/genesis.json"#, config.basefee, config.home, config.home))
        .spawn()?
        .wait()?;
    Ok(())
}

fn init() -> Result<(), Error> {
    let config = load_config()?;
    Command::new("evmosd")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg("init")
        .arg(config.moniker)
        .arg("-o")
        .arg("--chain-id")
        .arg(config.chain_id)
        .arg("--home")
        .arg(config.home)
        .arg("--overwrite")
        .spawn()?
        .wait()?;

    Ok(())
}

fn enable_api() -> Result<(), Error> {
    let config = load_config()?;
    let app_toml_path = Path::new(&config.home).join("config").join("app.toml");

    Command::new("sh")
        .arg("-c")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&format!(
            r#"sed -i.bak 's/enabled = false/enabled = true/g' {} && sed -i.bak 's/enable = false/enable = true/g' {}"#,
            app_toml_path.display(),
            app_toml_path.display()
        ))
        .spawn()?
        .wait()?;

    Ok(())
}

fn disable_rosetta_api() -> Result<(), Error> {
    let config = load_config()?;
    let app_toml_path = Path::new(&config.home).join("config").join("app.toml");

    Command::new("sh")
        .arg("-c")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&format!(
            r#"sed -i.bak '/\[rosetta\]/,/^\[/ s/enable = true/enable = false/' {}"#,
            app_toml_path.display()
        ))
        .spawn()?
        .wait()?;

    Ok(())
}

fn disable_memiavl() -> Result<(), Error> {
    let config = load_config()?;
    let app_toml_path = Path::new(&config.home).join("config").join("app.toml");

    Command::new("sh")
        .arg("-c")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&format!(
            r#"sed -i.bak '/\[memiavl\]/,/^\[/ s/enable = true/enable = false/' {}"#,
            app_toml_path.display()
        ))
        .spawn()?
        .wait()?;

    Ok(())
}

fn disable_versiondb() -> Result<(), Error> {
    let config = load_config()?;
    let app_toml_path = Path::new(&config.home).join("config").join("app.toml");

    Command::new("sh")
        .arg("-c")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&format!(
            r#"sed -i.bak '/\[versiondb\]/,/^\[/ s/enable = true/enable = false/' {}"#,
            app_toml_path.display()
        ))
        .spawn()?
        .wait()?;

    Ok(())
}

fn change_proposal_periods() -> Result<(), Error> {
    let config = load_config()?;
    let genesis_path = Path::new(&config.home).join("config").join("genesis.json");

    Command::new("sh")
        .arg("-c")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&format!(
            r#"jq '.app_state["gov"]["params"]["max_deposit_period"]="30s" | .app_state["gov"]["params"]["voting_period"]="30s" | .app_state["gov"]["params"]["expedited_voting_period"]="15s"' {} >/tmp/genesis.json && mv /tmp/genesis.json {}"#,
            genesis_path.display(),
            genesis_path.display()
        ))
        .spawn()?
        .wait()?;

    Ok(())
}

fn allocate_genesis_accounts() -> Result<(), Error> {
    let config = load_config()?;

    for account in config.genesis_accounts.iter() {
        Command::new("sh")
            .arg("-c")
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .arg(&format!(
                r#"evmosd add-genesis-account "$(evmosd keys show {} -a --keyring-backend {} --home {})" 100000000000000000000000000{} --keyring-backend {} --home {}"#,
                account.name,
                config.keyring_backend,
                config.home,
                config.base_denom,
                config.keyring_backend,
                config.home
            ))
            .spawn()?
            .wait()?;
    }

    Ok(())
}

fn sign_genesis_transaction() -> Result<(), Error> {
    let config = load_config()?;

    Command::new("evmosd")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg("gentx")
        .arg(config.genesis_accounts[0].name.clone())
        .arg(&format!("1000000000000000000000{}", config.base_denom))
        .arg("--gas-prices")
        .arg(format!("{}{}", config.basefee, config.base_denom))
        .arg("--keyring-backend")
        .arg(config.keyring_backend)
        .arg("--chain-id")
        .arg(config.chain_id)
        .arg("--home")
        .arg(config.home)
        .spawn()?
        .wait()?;
    Ok(())
}

fn collect_gentxs() -> Result<(), Error> {
    let config = load_config()?;
    Command::new("evmosd")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg("collect-gentxs")
        .arg("--home")
        .arg(config.home)
        .spawn()?
        .wait()?;
    Ok(())
}

fn validate_genesis() -> Result<(), Error> {
    let config = load_config()?;
    Command::new("evmosd")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg("validate-genesis")
        .arg("--home")
        .arg(config.home)
        .spawn()?
        .wait()?;
    Ok(())
}

fn start() -> Result<(), Error> {
    let config = load_config()?;
    Command::new("evmosd")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg("start")
        .arg("--home")
        .arg(config.home)
        .arg("--chain-id")
        .arg(config.chain_id)
        .arg(&format!(
            "--minimum-gas-prices={}{}",
            config.minimum_gas_prices, config.base_denom
        ))
        .arg("--json-rpc.api")
        .arg("eth,txpool,personal,net,debug,web3")
        .spawn()?
        .wait()?;
    Ok(())
}

fn load_config() -> Result<EvmosUpConfig, Error> {
    let config_path = "evmosup.toml";

    if !Path::new(config_path).exists() {
        return Ok(load_config()?);
    }

    let config_content = std::fs::read_to_string(config_path)
        .map_err(|e| Error::msg(format!("Failed to read config file: {}", e)))?;
    let config: EvmosUpConfig = toml::from_str(&config_content)
        .map_err(|e| Error::msg(format!("Failed to parse config file: {}", e)))?;
    Ok(config)
}
