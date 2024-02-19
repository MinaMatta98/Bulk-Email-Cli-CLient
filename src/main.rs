#![feature(let_chains)]
use clap::{CommandFactory, Parser};
use cli::{EmailInfo, Cli, Commands};
use email::Email;
use paste::paste;
use std::{env::VarError, error::Error, path::PathBuf};

mod cli;
mod email;
mod error;

fn get_var(key: &str) -> Result<String, VarError> {
    std::env::var(key).map_err(|e| {
        tracing::error!("Error Returned: {e} when looking for {key}");
        e
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();

    let args = Cli::parse();

    match args.command {
        Commands::SingleEmail(email_info) => {
            let EmailInfo {
                to_addr,
                subject,
                email_template,
                attachment,
            } = email_info;
            if let Err(e) = std::fs::metadata(&email_template) {
                Cli::command().error(
                    clap::error::ErrorKind::InvalidSubcommand, 
                    format!("failed to load metadata for --email-template path: {} with the following error: {e}", email_template.display())
                ).exit();
            }

            let email = Email::new(to_addr, subject, email_template, attachment);
            email.send_single_email().await?;
        }
        Commands::BulkEmail { csv_file } => {
            Email::send_bulk(csv_file).await?;
        }
    }

    Ok(())
}
