use super::config;
use clap::{Parser, Subcommand};
use log::info;

mod store;

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
pub struct AuthCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<AuthCommands>,
}

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Add a new API key
    #[clap(arg_required_else_help(true))]
    Add {
        #[clap(short, long)]
        interactive: bool,

        #[clap(long)]
        api_key: Option<String>,

        #[clap(long)]
        access_code: Option<String>,
    },
    /// Get Auth information
    Get {},
}

pub fn command(command: &AuthCommand) {
    match command.command.as_ref() {
        Some(AuthCommands::Add {
            interactive,
            api_key,
            access_code,
        }) => {
            if *interactive {
                store::interactive()
            }

            match api_key {
                Some(api_key) => store::api_key(api_key),
                None => {}
            }

            match access_code {
                Some(access_code) => store::access_code(access_code),
                None => {}
            }
        }
        Some(AuthCommands::Get {}) => {
            info!("Retrieving current configuration");
            info!("Live API Key: {:?}", config::api_key().ok());
            info!("Test API Key: {:?}", config::api_key_test().ok());
            info!("Access Token: {:?}", config::access_code().ok());
        }
        None => {}
    }
}
