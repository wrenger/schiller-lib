use crate::error::{Error, Result};

use email_address::EmailAddress;
use tracing::error;

/// Checks if the username is valid for an email
pub fn account_is_valid(account: &str) -> bool {
    EmailAddress::is_valid_local_part(account)
}
#[cfg(debug_assertions)]
pub fn send(
    _host: &str,
    _password: &str,
    _from: &str,
    _to: &str,
    _subject: &str,
    _body: &str,
) -> Result<()> {
    error!("Mail sending is disabled for debug builds");
    Err(Error::Network)
}

#[cfg(not(debug_assertions))]
pub fn send(
    host: &str,
    password: &str,
    from: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<()> {
    use lettre::message::{header::ContentType, Mailbox, SinglePartBuilder};
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Address, Message, SmtpTransport, Transport};
    use std::time::Duration;
    use tracing::info;
    use unicode_normalization::UnicodeNormalization;
    info!("Send mail to {to}");

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

impl From<lettre::address::AddressError> for Error {
    fn from(e: lettre::address::AddressError) -> Self {
        error!("Invalid Mail Address {e:?}");
        Error::Arguments
    }
}
impl From<lettre::error::Error> for Error {
    fn from(e: lettre::error::Error) -> Self {
        error!("Invalid Mail Format {e:?}");
        Error::Arguments
    }
}
impl From<lettre::transport::smtp::Error> for Error {
    fn from(e: lettre::transport::smtp::Error) -> Self {
        error!("Mail SMTP Error {e:?}");
        Error::Network
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
