use clap::{Arg, Command};

use crate::cmd::{init::init, reset::reset, up::up};

pub mod cmd;
pub mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("evmosup")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Tsiry Sandratraina <tsiry.sndr@rocksky.app>")
        .subcommand(
            Command::new("init")
                .about("Generate a init file for Evmos setup")
                .arg(
                    Arg::new("accounts")
                        .short('a')
                        .long("accounts")
                        .default_value("4")
                        .help("Number of accounts to generate for the Evmos setup"),
                ),
        )
        .subcommand(
            Command::new("reset")
                .about("Reset the Evmos blockchain, removing existing configurations"),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        let accounts = matches.get_one::<String>("accounts").unwrap();
        init(accounts.parse()?)?;
        return Ok(());
    }

    if let Some(_) = matches.subcommand_matches("reset") {
        reset()?;
        return Ok(());
    }

    up()?;

    Ok(())
}
