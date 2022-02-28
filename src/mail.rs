use std::time::Duration;

use crate::api;

use lettre::message::{header::ContentType, Mailbox, SinglePartBuilder};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Address, Message, SmtpTransport, Transport};
use unicode_normalization::UnicodeNormalization;

pub fn send(
    host: &str,
    password: &str,
    from: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> api::Result<()> {
    // Change encoding of äöü to ascii
    let subject = subject.nfc().collect::<String>();
    let body = body.nfc().collect::<String>();

    // Create mail
    let email = Message::builder()
        .from(Mailbox::new(None, Address::new(from, host)?))
        .to(Mailbox::new(None, Address::new(to, host)?))
        .subject(subject)
        .singlepart(
            SinglePartBuilder::new()
                .content_type(ContentType::TEXT_PLAIN)
                .body(body),
        )?;

    // Open tls encrypted smtp connection
    let mailer = SmtpTransport::relay(host)?
        .credentials(Credentials::new(from.to_string(), password.to_string()))
        .timeout(Some(Duration::from_secs(1)))
        .build();

    // Send the email
    mailer.send(&email)?;
    Ok(())
}

impl From<lettre::address::AddressError> for api::Error {
    fn from(e: lettre::address::AddressError) -> Self {
        gdnative::godot_error!("Invalid Mail Address {:?}", e);
        api::Error::Arguments
    }
}
impl From<lettre::error::Error> for api::Error {
    fn from(e: lettre::error::Error) -> Self {
        gdnative::godot_error!("Invalid Mail Format {:?}", e);
        api::Error::Arguments
    }
}
impl From<lettre::transport::smtp::Error> for api::Error {
    fn from(e: lettre::transport::smtp::Error) -> Self {
        gdnative::godot_error!("Mail SMTP Error {:?}", e);
        api::Error::Network
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn send_mail() {
        super::send(
            &std::env::var("SBV_MAIL_HOST").unwrap(),
            &std::env::var("SBV_MAIL_PASSWORD").unwrap(),
            &std::env::var("SBV_MAIL_FROM").unwrap(),
            &std::env::var("SBV_MAIL_TO").unwrap(),
            "Test Mail 🚧",
            "Test Content 🚧",
        )
        .unwrap();
    }
}
