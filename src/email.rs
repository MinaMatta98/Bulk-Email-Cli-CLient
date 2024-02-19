use crate::cli::AttachmentCommand;
pub use crate::error::EmailingError;
use crate::get_var;
use crate::paste;
use crate::EmailInfo;
use crate::PathBuf;
use lettre::message::Attachment;
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! email_struct {

    (2) => {
        email_struct!(1,2);
    };

    (3) => {
        email_struct!(1,2,3);
    };

    (4) => {
        email_struct!(1,2,3,4);
    };

    (5) => {
        email_struct!(1,2,3,4,5);
    };

    (6) => {
        email_struct!(1,2,3,4,5,6);
    };

    (7) => {
        email_struct!(1,2,3,4,5,6,7);
    };

    (8) => {
        email_struct!(1,2,3,4,5,6,7,8);
    };

    (9) => {
        email_struct!(1,2,3,4,5,6,7,8,9);
    };

    (10) => {
        email_struct!(1,2,3,4,5,6,7,8,9,10);
    };

    ( $( $no:expr ),* ) => {
            paste! {
                #[derive(Debug, Deserialize, Serialize)]
                pub struct Email {
                    delivery_address: String,
                    subject: String,
                    email_body_location: PathBuf,
                    #[serde(flatten)]
                    attachment: AttachmentCommand,
                }

                impl Email {
                    #[allow(clippy::too_many_arguments)]
                    pub fn new(
                            delivery_address: String,
                            subject: String,
                            email_body_location: PathBuf,
                            attachment: AttachmentCommand,
                    ) -> Self {
                                Email {
                                    delivery_address,
                                    subject,
                                    email_body_location,
                                    attachment
                                }
                    }

                    pub async fn send_bulk(path: PathBuf) -> Result<(), $crate::email::EmailingError> {
                        let mut reader = csv::Reader::from_path(path)?;
                        let email_info = reader.deserialize::<EmailInfo>();
                        let mut join_set = tokio::task::JoinSet::new();

                        for (index, email) in email_info.enumerate() {
                            match email {
                                Ok(email) => {
                                    let EmailInfo {
                                        to_addr,
                                        subject,
                                        email_template,
                                        attachment,
                                    } = email;
                                    let email = Email::new(to_addr, subject, email_template, attachment);
                                    join_set.spawn(async move { (email.send_single_email().await, index) });
                                }
                                Err(e) => {
                                    tracing::error!("Error occured on line {}: {e} ", index + 2)
                                }
                            }
                        }

                        while let Some(res) = join_set.join_next().await {
                            match res? {
                                (Ok(_), _) => (),
                                (Err(e), index) => tracing::error!("{e} at line no: {}", index + 2),
                            }
                        }
                        Ok(())
                    }


                pub async fn send_single_email(self) -> Result<(), EmailingError> {
                    use lettre::{
                        message::{
                            header::{self, ContentType},
                            MultiPart, SinglePart,
                        },
                        transport::smtp::authentication::Credentials,
                        AsyncSmtpTransport, AsyncTransport, Message,
                    };

                        let Email {
                            delivery_address,
                            subject,
                            email_body_location,
                            attachment,
                        } = self;

                        let password = get_var("SENDER_PASSWORD")?;
                        let from_addr = get_var("SENDER_EMAIL")?;
                        let smtp_relay = get_var("SMTP_RELAY")?;

                        let mut email = std::fs::read_to_string(email_body_location)?;

                        #[allow(clippy::collapsible_match)]
                        let AttachmentCommand::Attachments {
                            $(
                                    [<attachment_ $no _path>],
                                    [<attachment_ $no _inline_content_id>],
                            )*
                            replacement_tokens
                        }
                         = attachment;

                        replacement_tokens.replace_all_substrings(&mut email);

                        let mut multi_part = MultiPart::related().singlepart(
                            SinglePart::builder()
                                .header(header::ContentType::TEXT_HTML)
                                .body(email),
                        );

                        $(
                          if let Some([<attachment_ $no _path>]) = [<attachment_ $no _path>] && &[<attachment_ $no _path>].display().to_string() != "" {
                                let attachment = if let Some([<attachment_ $no _inline_val>]) = [<attachment_ $no _inline_content_id>] && [<attachment_ $no _inline_val>] != String::new() {
                                    Attachment::new_inline(
                                            [<attachment_ $no _inline_val>].as_number().unwrap().to_string()
                                    )
                                    .body(
                                        std::fs::read(&[< attachment_ $no _path>])?,
                                        ContentType::parse(&mime_guess::from_path([< attachment_ $no _path>]).first().unwrap().to_string())?,
                                    )

                                } else {
                                    Attachment::new([< attachment_ $no _path>].file_name().unwrap().to_str().unwrap().to_string())
                                    .body(
                                        std::fs::read(&[< attachment_ $no _path>])?,
                                        ContentType::parse(&mime_guess::from_path([< attachment_ $no _path>]).first().unwrap().to_string())?,
                                    )

                                };
                            multi_part = multi_part.singlepart(attachment);
                          };
                        )*

                        let email = Message::builder()
                            .from(from_addr.parse()?)
                            .to(delivery_address.parse()?)
                            .subject(subject)
                            .multipart(multi_part)?;

                        let creds = Credentials::new(from_addr, password);

                        let mailer = AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(&smtp_relay)
                            .unwrap()
                            .credentials(creds)
                            .build();

                        match mailer.send(email).await {
                            Ok(_) => tracing::info!("Email sent successfully!"),
                            Err(e) => tracing::error!("Could not send email: {e:?}"),
                        };

                        Ok(())
                    }
                }
            }
    };

}

email_struct!(10);
