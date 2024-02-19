use crate::tokens::ReplacementTokens;
use clap::{arg, command, Parser, Subcommand};
use paste::paste;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{path::PathBuf, u64};

#[derive(Parser)]
#[command(version, about = "
This is a cli emailing client

To use this client, ensure that the following environmental variables are configured:

    SENDER_EMAIL,
    SENDER_PASSWORD,
    # This is valid for gmail 
    SMTP_RELAY='smtp.gmail.com'
", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[macro_export]
macro_rules! commands_struct {

    (2) => {
        commands_struct!(1,2);
    };

    (3) => {
        commands_struct!(1,2,3);
    };

    (4) => {
        commands_struct!(1,2,3,4);
    };

    (5) => {
        commands_struct!(1,2,3,4,5);
    };

    (6) => {
        commands_struct!(1,2,3,4,5,6);
    };

    (7) => {
        commands_struct!(1,2,3,4,5,6,7);
    };

    (8) => {
        commands_struct!(1,2,3,4,5,6,7,8);
    };

    (9) => {
        commands_struct!(1,2,3,4,5,6,7,8,9);
    };

    (10) => {
        commands_struct!(1,2,3,4,5,6,7,8,9,10);
    };

    ( $( $no:expr ),* ) => {
            paste! {
                #[derive(Debug, Serialize, Deserialize, Subcommand, Clone)]
                #[serde(untagged)]
                /// Add an attachment to the email with an inline_number
                pub enum AttachmentCommand {
                    #[serde(untagged)]
                    #[serde(rename_all = "PascalCase")]
                    Attachments {
                        $(
                          #[arg(long)]
                          #[serde(skip_serializing_if = "Option::is_none")]
                          [< attachment_ $no _path >]: Option<PathBuf>,

                          #[arg(long)]
                          #[serde(skip_serializing_if = "Option::is_none")]
                          [< attachment_ $no _inline_content_id >]: Option<Value>,

                        )*

                          #[serde(flatten)]
                          #[command(subcommand)]
                          replacement_tokens: Option<ReplacementTokens>,
                    },
                }
            }
    };

}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Commands {
    /// Sends a single email
    SingleEmail(EmailInfo),
    /// Sends bulk emails as structured within CSV file
    BulkEmail {
        /// CSV File Path
        #[arg(short, long)]
        csv_file: PathBuf,
    },
}

/// CSV file headers expected to fit the following
#[derive(serde::Serialize, serde::Deserialize, clap::Args, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EmailInfo {
    /// Address for the email To header
    #[serde(rename = "DeliveryAddress")]
    #[arg(short, long)]
    pub to_addr: String,

    /// Email subject header
    #[arg(short, long)]
    pub subject: String,

    /// Email template. This is normally a html email or string email
    #[serde(rename = "RelativeEmailTemplatePath")]
    #[arg(short, long)]
    pub email_template: PathBuf,

    /// Optional Inline File Attachment
    #[serde(flatten)]
    #[command(subcommand)]
    pub attachment: Option<AttachmentCommand>,
}

commands_struct!(10);

#[cfg(test)]
mod test {
    use std::error::Error;

    use super::{AttachmentCommand, EmailInfo};

    #[test]
    fn test_email_structure() -> Result<(), Box<dyn Error>> {
        let mut reader = csv::Reader::from_path("./example/test.csv")?;
        let email_info = reader.deserialize::<EmailInfo>();

        for record in email_info.into_iter() {
            let record = record.unwrap();
            tracing::info!("\n{}\n", serde_json::to_string_pretty(&record).unwrap());
            let AttachmentCommand::Attachments {
                attachment_1_path,
                attachment_1_inline_content_id,
                ..
            } = record.attachment.unwrap();

            assert!(attachment_1_path.is_some());
            assert!(attachment_1_inline_content_id.is_some());
        }

        Ok(())
    }
}
