use clap::Parser;
use clap::Subcommand;
use email_address::EmailAddress;

use std::str::FromStr;

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Confirm {
        #[arg(short, long)]
        user_addr: String,

        #[arg(short, long)]
        wks_addr: String,

        #[arg(short, long)]
        nonce: String,

        #[arg(short, long)]
        key: Option<String>
    }
}

fn main() -> Result<(), &'static str> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Confirm { user_addr, wks_addr, nonce, key }) => {
            let user_email_addr = match EmailAddress::from_str(user_addr) {
                Ok(addr) => addr,
                Err(_) => return Err("Error parsing user email address.")
            };
            let wks_email_addr = match EmailAddress::from_str(wks_addr) {
                Ok(addr) => addr,
                Err(_) => return Err("Error parsing submission email address.")
            };

            match key {
                Some(key) => commands::confirm::confirm_encrypt(user_email_addr, wks_email_addr, nonce, key),
                None => commands::confirm::confirm_plaintext(user_email_addr, wks_email_addr, nonce)
            }
        },
        None => Err("No subcommand provided.")
    }
}
