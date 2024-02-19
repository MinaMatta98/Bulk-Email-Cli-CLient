use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use struct_iterable::Iterable;

#[derive(Subcommand, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(untagged)]
pub enum ReplacementTokens {
    Token {
        #[command(flatten)]
        #[serde(flatten)]
        replacement_tokens: Tokens,
    },
}

#[derive(clap::Args, Debug, Serialize, Deserialize, Iterable, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Tokens {
    /// Replace instance of {{name}} within email-template to --name value
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Value>,

    /// Replace instance of {{email}} within email-template to --email value
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<Value>,

    /// Replace instance of {{date}} within email-template to --date value
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<Value>,

    /// Replace instance of {{time}} within email-template to --time value
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<Value>,

    /// Replace instance of {{order_number}} within email-template to --order-number value
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    order_number: Option<Value>,

    /// Replace instance of {{product_information}} within email-template to --product-information value
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    product_information: Option<Value>,

    /// Replace instance of {{location}} within email-template to --location value
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<Value>,

    /// Replace instance of {{user_name}} within email-template to --user-name value
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<Value>,

    /// Replace instance of {{confirmation_link}} within email-template to --confirmation-link value
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    confirmation_link: Option<Value>,

    /// Replace instance of {{password_reset_link}} within email-template to --password-reset-link value
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    password_reset_link: Option<Value>,

    /// Replace instance of {{discount_code}} within email-template to --discount-code value
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    discount_code: Option<Value>,

    /// Replace instance of {{temp_code}} within email-template to --temp-code value
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    temp_code: Option<Value>,

    /// Replace instance of {{course_details}} within email-template to --course-details value
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    course_details: Option<Value>,
}

impl ReplacementTokens {
    pub fn replace_all_substrings(&self, raw_string: &mut String) {
        let ReplacementTokens::Token { replacement_tokens } = self;

        for (key, val) in replacement_tokens.iter() {
            let key = "{{".to_string() + key + "}}";
            if let Some(val) = val.downcast_ref::<Option<Value>>() {
                if let Some(val) = val
                    && val != &String::new()
                {
                    if let Some(val) = val.as_str() {
                        *raw_string = raw_string.replace(&key, val);
                    } else if let Some(val) = val.as_number() {
                        let val = val.to_string();
                        *raw_string = raw_string.replace(&key, val.as_str());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ReplacementTokens, Tokens};
    use crate::cli::{AttachmentCommand, EmailInfo};
    use std::error::Error;

    #[tracing::instrument]
    #[test]
    fn test_replace_all_substrings() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt().init();
        let replacement_tokens = Tokens {
            name: Some("Mina".into()),
            temp_code: Some("123456".into()),
            ..Default::default()
        };

        replace_html_file_substrings(replacement_tokens)?;
        Ok(())
    }

    fn replace_html_file_substrings(replacement_tokens: Tokens) -> Result<(), Box<dyn Error>> {
        let mut string = std::fs::read_to_string("./templates/replacement_test.html")?;
        let replacement_tokens = ReplacementTokens::Token { replacement_tokens };

        replacement_tokens.replace_all_substrings(&mut string);

        assert_eq!(
            string,
            r#"<body>
	<div style="display: flex; justify-content: center;">
		<img src="cid:2335" style="margin: 50px auto 10px auto; width: 100px">
	</div>
	<div class="container">
		<h1 style="text-align: center">Email Sender Template</h1>
			<p>Thank you Mina for using email-sender. The verification code is: </p>
		<div style="justify-items: center;" onclick="copyToClipboard()">
			<div class="validation-container">
				<p style="margin: auto; text-align: center" class="validation" id="text">
						123456
				</p>
			</div>
		</div>
	</div>
	<div class="footer">
		&copy;
		<a href="">
			2023 Company PTY LTD.
		</a>
		All rights reserved.
	</div>
</body>
"#
        );
        Ok(())
    }

    #[tracing::instrument]
    #[test]
    fn replace_all_substrings_csv() -> Result<(), Box<dyn Error>> {
        let mut reader = csv::Reader::from_path("./example/test.csv")?;
        let email_info = reader.deserialize::<EmailInfo>();

        for record in email_info.into_iter() {
            let record = record.unwrap();

            let AttachmentCommand::Attachments {
                replacement_tokens, ..
            } = record.attachment;

            let ReplacementTokens::Token { replacement_tokens } = replacement_tokens;

            replace_html_file_substrings(replacement_tokens)?;
        }

        Ok(())
    }
}
