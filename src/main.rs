use clap::{Arg, Command};

use crate::cmd::{init::init, reset::reset, up::up};

pub mod cmd;
pub mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("evmosup")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Tsiry Sandratraina <tsiry.sndr@rocksky.app>")
        .subcommand(Command::new("init").about("Generate a init file for Evmos setup"))
        .subcommand(
            Command::new("reset")
                .about("Reset the Evmos blockchain, removing existing configurations")
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Force reset without confirmation"),
                ),
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("init") {
        init()?;
        return Ok(());
    }

    if let Some(_) = matches.subcommand_matches("reset") {
        reset()?;
        return Ok(());
    }

    up()?;

    Ok(())
}
