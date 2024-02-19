[![Rust Build](https://github.com/MinaMatta98/Bulk-Email-Cli-CLient/actions/workflows/rust.yml/badge.svg)](https://github.com/MinaMatta98/Bulk-Email-Cli-CLient/actions/workflows/rust.yml)

[![Test](https://github.com/MinaMatta98/Bulk-Email-Cli-CLient/actions/workflows/test.yml/badge.svg)](https://github.com/MinaMatta98/Bulk-Email-Cli-CLient/actions/workflows/test.yml)


[![GitHub version](https://img.shields.io/github/v/tag/MinaMatta98/Bulk-Email-Cli-CLient?label=Version)](https://github.com/MinaMatta98/Bulk-Email-Cli-CLient/releases)



<div align="center">
    <img src="./example/send-mail-7590.svg"/>
</div>

<h1 align="center">
    Bulk Email Cli Client
</h1>

An asynchronous [RUST](https://www.rust-lang.org/) based CLI bulk emailing client, built on top of [lettre-rs](https://github.com/lettre/lettre).


## Features

|Feature|Description|
|-------|-----------|
|Template Variable Substitution| Refer to the [Template Variable Substitution](#template-variable-substitution) section for modifying email-templates|
|Single Email Support| Refer to [Send a Single Email](#send-a-single-email)|
|Bulk Email Support| Support for asynchronous unordered sending of bulk emails detailed within a csv file as described within [Send Bulk Emails](#send-bulk-emails) |
|Attachment Support| Support for Inline/Standard Attachments |
|SMTP Transport| This tool uses the SMTP protocol to send emails over the network |
|TLS Encryption| This tool defaults to TLS encryption over network|

<br/>
<br/>

## Instructions:

### Installation:
Installation of this tool will require [RUST installation version](https://www.rust-lang.org/tools/install) of 1.70 or newer.

To install this package via cargo, run the following command from a shell of choice:

```bash
cargo install --locked --git https://github.com/MinaMatta98/Bulk-Email-Cli-CLient.git
```
<br/>
<br/>

### Setting Environmental Variables
---
The following environmental variables must be set:

|Variable|Description|
|-------|-----------|
|SENDER_EMAIL|Email of the email account sending the Email(s) |
|SENDER_PASSWORD|Corrosponding password for the email account sending Email(s).Turn on Less-Secure-Apps to send emails: https://myaccount.google.com |
|SMTP_RELAY|The SMTP relay corrosponding to the SMTP Gateway of Choice. For example, Gmail uses smtp.gmail.com and AOL uses smtp.aol.com |

<br/>
<br/>

### Send a Single Email
---
Refer to [Environmental Variables](#environmental-variables) for setting up environmental variables.


#### Scenario 1
Send an email to `minamatta98@gmail.com` with the template saved within `./templates/email.html` relative to the current working directory (check with ```bash pwd```).

```bash
email-sender single-email --to-addr "minamatta98@gmail.com" --subject "Email Testing CLI" --email-template "./templates/email.html"
```

<br/>

#### Scenario 2
Send an email to `minamatta98@gmail.com` with the template saved within `./templates/email.html` relative to the current working directory (check with ```bash pwd```).
This is to also send a pdf attachment within the example directory relative to the root directory of this repository.

```bash
email-sender single-email --to-addr "minamatta98@gmail.com" --subject "Email Testing CLI" --email-template "./templates/email.html" attachments --attachment-1-path "./example/rust.pdf"
```
<br/>

#### Scenario 3
Send an email to `minamatta98@gmail.com` with the template saved within `./templates/email.html` relative to the current working directory (check with ```bash pwd```).
This is to also send an inline email attachment located within `./example/send_mail_7590.png` corresponding to an inline content id of 2335.

```bash
email-sender single-email --to-addr "minamatta98@gmail.com" --subject "Email Testing CLI" --email-template "./templates/email.html" attachments --attachment-1-path "./example/send-mail-7590.svg" --attachment-1-inline-content-id 2335
```


<br/>

##### Result

<div align="center">
    <img src="./Showcase/Example Email.png"/>
</div>

<br/>
<br/>
<br/>
<br/>

### Send Bulk Emails
---
All three scenarios are listed within their respective order within the following table.

This will correspond to the following csv file:

> _Note that unfilled fields must still be comma separated._
> _Also note that attachment and substitution headers can be omitted where not needed_

```csv
DeliveryAddress,Subject,RelativeEmailTemplatePath,Attachment1Path,Attachment1InlineContentId
minamatta98@gmail.com,Email Template Tests,./templates/email.html,,
deran_lockon@hotmail.com,Email Template Tests,./templates/email.html,./example/rust.pdf,
minamatta98@gmail.com,Email Template Tests,./templates/email.html,./example/send_mail_7590.png,2335
```

To send bulk emails from a csv file saved within `./example/example.csv`, run the following command from your shell of choice:

```bash
email-sender bulk-email --csv-file "./example/example.csv"
```

<br/>
<br/>

### Template Variable Substitution
---
The following variables can be used within an email-template and encoded as `{{field name}}` to be replaced:

| Field Name           | Description                                                                                          |Csv Header |
|----------------------|------------------------------------------------------------------------------------------------------| ----------|
| `name`               | Replace instance of `{{name}}` within email-template to `--name` value                                |Name|
| `email`              | Replace instance of `{{email}}` within email-template to `--email` value                              |Email|
| `date`               | Replace instance of `{{date}}` within email-template to `--date` value                                |Date|
| `time`               | Replace instance of `{{time}}` within email-template to `--time` value                                |Time|
| `order_number`       | Replace instance of `{{order_number}}` within email-template to `--order-number` value                |OrderNumber|
| `product_information`| Replace instance of `{{product_information}}` within email-template to `--product-information` value| ProductInformation |
| `location`           | Replace instance of `{{location}}` within email-template to `--location` value                        |Location|
| `username`           | Replace instance of `{{user_name}}` within email-template to `--user-name` value                      |Username|
| `confirmation_link`  | Replace instance of `{{confirmation_link}}` within email-template to `--confirmation-link` value      |ConfirmationLink|
| `password_reset_link`| Replace instance of `{{password_reset_link}}` within email-template to `--password-reset-link` value |PasswordResetLink|
| `discount_code`      | Replace instance of `{{discount_code}}` within email-template to `--discount-code` value              |DiscountCode|
| `temp_code`          | Replace instance of `{{temp_code}}` within email-template to `--temp-code` value                      |TempCode|
| `course_details`     | Replace instance of `{{course_details}}` within email-template to `--course-details` value           |CourseDetails|

<br/>

#### Single Email Substitution Example
In this example, instances of `{{name}}` within the email-template will be replaced with Mina and instances of ``{{temp_code}}`` will be replaced with 123456

```bash
cargo run --release -- single-email --to-addr "minamatta98@gmail.com" --subject "Email Testing CLI" --email-template "./templates/test.html" attachments --attachment-1-path "./example/send-mail-7590.svg" --attachment-1-inline-content-id 2335 token --name "Mina" --temp-code 123456
```

<br/>

#### Bulk Email Substitution Example
The following example accomplishes the same function as the [single email substitution example](#single-email-substitution-example). 

> _Note that variables are evaluated separately for each row entry and can be left blank if not needed._
> _Also note that field names for substitutions can be left empty if not needed. For example, the Name and TempCode header are not required if values are not passed._

```csv
DeliveryAddress,Subject,RelativeEmailTemplatePath,Attachment1Path,Attachment1InlineContentId,Name,TempCode
minamatta98@gmail.com,Email Template Tests,./templates/replacement_test.html,./example/send_mail_7590.png,2335,Mina,123456
```
