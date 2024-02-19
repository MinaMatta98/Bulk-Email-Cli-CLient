<div style="justify-content: center; display:flex;">
    <img src="./example/send-mail-7590.svg"/>
</div>
<h1 style="text-align: center;">
    Bulk Email Cli Client
</h1>

An asynchronous [RUST](https://www.rust-lang.org/) based CLI bulk emailing client, built on top of [lettre-rs](https://github.com/lettre/lettre).

### Features:

|Feature|Description|
|-------|-----------|
|Single Email Support| Refer to [Send a Single Email](#send-a-single-email)|
|Bulk Email Support| Support for asynchronous unordered sending of bulk emails detailed within a csv file as described within [Send Bulk Emails](#send-bulk-emails) |
|Attachment Support| Support for Inline/Standard Attachments |
|SMTP Transport| This tool uses the SMTP protocol to send emails over the network |
|TLS Encryption| This tool defaults to TLS encryption over network|

<br/>
<br/>
<br/>
<br/>

### Instructions:

#### Installation:
Installation of this tool will require [RUST installation version](https://www.rust-lang.org/tools/install) of 1.70 or newer.

To install this package via cargo, run the following command from a shell of choice:

```bash
    cargo install --locked --git https://github.com/MinaMatta98/Bulk-Email-Cli-CLient.git
```

<br/>
<br/>
<br/>
<br/>

#### Setting Environmental Variables
The following environmental variables must be set:

|Variable|Description|
|-------|-----------|
|SENDER_EMAIL|Email of the email account sending the Email(s) |
|SENDER_PASSWORD|Corrosponding password for the email account sending Email(s)|
|SMTP_RELAY|The SMTP relay corrosponding to the SMTP Gateway of Choice. For example, Gmail uses smtp.gmail.com and AOL uses smtp.aol.com |

<br/>
<br/>
<br/>
<br/>

#### Send a Single Email
Refer to [Environmental Variables](#environmental-variables) for setting up environmental variables.


##### Scenario 1:
Send an email to `minamatta98@gmail.com` with the template saved within `./templates/email.html` relative to the current working directory (check with ```bash pwd```).

```bash
email-sender single-email --to-addr "minamatta98@gmail.com" --subject "Email Testing CLI" --email-template "./templates/email.html"
```

<br/>

##### Scenario 2:
Send an email to `minamatta98@gmail.com` with the template saved within `./templates/email.html` relative to the current working directory (check with ```bash pwd```).
This is to also send a pdf attachment within the example directory relative to the root directory of this repository.

```bash
email-sender single-email --to-addr "minamatta98@gmail.com" --subject "Email Testing CLI" --email-template "./templates/email.html" --attachment-1-path "./example/rust.pdf"
```
<br/>

##### Scenario 3:
Send an email to `minamatta98@gmail.com` with the template saved within `./templates/email.html` relative to the current working directory (check with ```bash pwd```).
This is to also send an inline email attachment located within `./P-Circle.png` corrosponding to an inline content id of 2335.

```bash
email-sender single-email --to-addr "minamatta98@gmail.com" --subject "Email Testing CLI" --email-template "./templates/email.html" --attachment-1-path "./P-Circle.png" --attachment-1-inline-content-id 2335
```

<br/>
<br/>
<br/>
<br/>

#### Send Bulk Emails
All three scenarios are listed within their respective order within the following table.

This will corrospond to the following csv file:

_Note that unfilled fields must still be comma seperated_

```csv
DeliveryAddress,Subject,RelativeEmailTemplatePath,Attachment1Path,Attachment1InlineContentId
minamatta98@gmail.com,Email Template Tests,./templates/email.html,,
deran_lockon@hotmail.com,Email Template Tests,./templates/email.html,./example/rust.pdf,
minamatta98@gmail.com,Email Template Tests,./templates/email.html,./example/P-Circle.png,2335
```

To send bulk emails from a csv file saved within `./example/example.csv`, run the following command from your shell of choice:

```bash
cargo run --release -- bulk-email --csv-file "./example/example.csv"
```
