use anyhow::Error;
use owo_colors::OwoColorize;
use std::process::Command;

const CHAIN_ID: &str = "evmos_9002-1";
const KEYRING_BACKEND: &str = "test";
const KEY_ALGO: &str = "eth_secp256k1";
const BASE_DENOM: &str = "aevmos";
const MONIKER: &str = "localtestnet";
const BASEFEE: u64 = 1000000000;
const VAL_KEY: &str = "mykey";
const USER1_KEY: &str = "dev0";
const USER2_KEY: &str = "dev1";
const USER3_KEY: &str = "dev2";
const USER4_KEY: &str = "dev3";

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
    change_pruning_settings()?;
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
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");

    Command::new("sh")
        .arg("-c")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&format!(
            "evmosd config set client chain-id {} --home {} && evmosd config set client keyring-backend {} --home {}",
            CHAIN_ID,
            evmos_home.display(),
            KEYRING_BACKEND,
            evmos_home.display()
        ))
        .spawn()?
        .wait()?;

    Ok(())
}

fn import_keys() -> Result<(), Error> {
    let val_key = VAL_KEY;
    let val_mnemonic = "gesture inject test cycle original hollow east ridge hen combine junk child bacon zero hope comfort vacuum milk pitch cage oppose unhappy lunar seat";

    let user1_key = USER1_KEY;
    let user1_mnemonic = "copper push brief egg scan entry inform record adjust fossil boss egg comic alien upon aspect dry avoid interest fury window hint race symptom";

    let user2_key = USER2_KEY;
    let user2_mnemonic = "maximum display century economy unlock van census kite error heart snow filter midnight usage egg venture cash kick motor survey drastic edge muffin visual";

    let user3_key = USER3_KEY;
    let user3_mnemonic = "will wear settle write dance topic tape sea glory hotel oppose rebel client problem era video gossip glide during yard balance cancel file rose";

    let user4_key = USER4_KEY;
    let user4_mnemonic = "doll midnight silk carpet brush boring pluck office gown inquiry duck chief aim exit gain never tennis crime fragile ship cloud surface exotic patch";

    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");

    let keys = [
        (val_key, val_mnemonic),
        (user1_key, user1_mnemonic),
        (user2_key, user2_mnemonic),
        (user3_key, user3_mnemonic),
        (user4_key, user4_mnemonic),
    ];

    for (key, mnemonic) in keys.iter() {
        Command::new("sh")
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .arg("-c")
            .arg(format!(
                "echo '{}' | evmosd keys add '{}' --recover --keyring-backend {} --algo {} --home {}",
                mnemonic,
                key,
                KEYRING_BACKEND,
                KEY_ALGO,
                evmos_home.display()
            ))
            .spawn()?
            .wait()?;
    }

    Ok(())
}

fn set_gas_limit() -> Result<(), Error> {
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");
    Command::new("sh")
        .arg("-c")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&format!(r#"jq '.consensus_params["block"]["max_gas"]="10000000"' {}/config/genesis.json >/tmp/genesis.json && mv /tmp/genesis.json {}/config/genesis.json"#, evmos_home.display(), evmos_home.display()))
        .spawn()?
        .wait()?;
    Ok(())
}

fn set_base_fee() -> Result<(), Error> {
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");
    Command::new("sh")
        .arg("-c")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&format!(r#"jq '.app_state["feemarket"]["params"]["base_fee"]='{}' {}/config/genesis.json >/tmp/genesis.json && mv /tmp/genesis.json {}/config/genesis.json"#, BASEFEE, evmos_home.display(), evmos_home.display()))
        .spawn()?
        .wait()?;
    Ok(())
}

fn init() -> Result<(), Error> {
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");
    Command::new("evmosd")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg("init")
        .arg(MONIKER)
        .arg("-o")
        .arg("--chain-id")
        .arg(CHAIN_ID)
        .arg("--home")
        .arg(evmos_home)
        .arg("--overwrite")
        .spawn()?
        .wait()?;

    Ok(())
}

fn enable_api() -> Result<(), Error> {
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");
    let app_toml_path = evmos_home.join("config").join("app.toml");

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
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");
    let app_toml_path = evmos_home.join("config").join("app.toml");

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
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");
    let app_toml_path = evmos_home.join("config").join("app.toml");

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
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");
    let app_toml_path = evmos_home.join("config").join("app.toml");

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
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");
    let genesis_path = evmos_home.join("config").join("genesis.json");

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

fn change_pruning_settings() -> Result<(), Error> {
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");
    let app_toml_path = evmos_home.join("config").join("app.toml");

    Command::new("sh")
        .arg("-c")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&format!(
            r#"sed -i.bak 's/pruning = "default"/pruning = "custom"/g' {} && sed -i.bak 's/pruning-keep-recent = "0"/pruning-keep-recent = "2"/g' {} && sed -i.bak 's/pruning-interval = "0"/pruning-interval = "10"/g' {}"#,
            app_toml_path.display(),
            app_toml_path.display(),
            app_toml_path.display()
        ))
        .spawn()?
        .wait()?;

    Ok(())
}

fn allocate_genesis_accounts() -> Result<(), Error> {
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");

    for key in [VAL_KEY, USER1_KEY, USER2_KEY, USER3_KEY, USER4_KEY].iter() {
        Command::new("sh")
            .arg("-c")
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .arg(&format!(
                r#"evmosd add-genesis-account "$(evmosd keys show {} -a --keyring-backend {} --home {})" 100000000000000000000000000{} --keyring-backend {} --home {}"#,
                key, KEYRING_BACKEND, evmos_home.display(), BASE_DENOM, KEYRING_BACKEND, evmos_home.display()
            ))
            .spawn()?
            .wait()?;
    }

    Ok(())
}

fn sign_genesis_transaction() -> Result<(), Error> {
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");
    Command::new("evmosd")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg("gentx")
        .arg(VAL_KEY)
        .arg(&format!("1000000000000000000000{}", BASE_DENOM))
        .arg("--gas-prices")
        .arg(format!("{}{}", BASEFEE, BASE_DENOM))
        .arg("--keyring-backend")
        .arg(KEYRING_BACKEND)
        .arg("--chain-id")
        .arg(CHAIN_ID)
        .arg("--home")
        .arg(evmos_home)
        .spawn()?
        .wait()?;
    Ok(())
}

fn collect_gentxs() -> Result<(), Error> {
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");
    Command::new("evmosd")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg("collect-gentxs")
        .arg("--home")
        .arg(evmos_home)
        .spawn()?
        .wait()?;
    Ok(())
}

fn validate_genesis() -> Result<(), Error> {
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");
    Command::new("evmosd")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg("validate-genesis")
        .arg("--home")
        .arg(evmos_home)
        .spawn()?
        .wait()?;
    Ok(())
}

fn start() -> Result<(), Error> {
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");
    Command::new("evmosd")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg("start")
        .arg("--home")
        .arg(evmos_home)
        .arg("--chain-id")
        .arg(CHAIN_ID)
        .arg(&format!("--minimum-gas-prices=0.0001{}", BASE_DENOM))
        .arg("--json-rpc.api")
        .arg("eth,txpool,personal,net,debug,web3")
        .spawn()?
        .wait()?;
    Ok(())
}
