use anyhow::Error;
use owo_colors::OwoColorize;

pub fn reset() -> Result<(), Error> {
    println!("{}", "Resetting Evmos blockchain...".yellow());
    let home = dirs::home_dir().unwrap();
    let evmos_home = home.join(".evmosd");

    match evmos_home.exists() {
        true => match std::fs::remove_dir_all(&evmos_home) {
            Ok(_) => {
                println!("{}", "Evmos blockchain reset successfully.".green());
            }
            Err(e) => {
                println!(
                    "{}",
                    format!("Failed to reset Evmos blockchain: {}", e).red()
                );
            }
        },
        false => {
            println!("{}", "No Evmos blockchain found, nothing to reset.".red());
        }
    }

    Ok(())
}
