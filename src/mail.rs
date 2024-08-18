use std::time::Duration;

use email_address::EmailAddress;
use lettre::message::header::ContentType;
use lettre::message::{Mailbox, SinglePartBuilder};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use tracing::{error, info};
use unicode_normalization::UnicodeNormalization;

use crate::error::{Error, Result};

/// Checks if the username is valid for an email
pub fn account_is_valid(account: &str) -> bool {
    EmailAddress::is_valid_local_part(account)
}

pub async fn send(
    host: &str,
    password: &str,
    from: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<()> {
    info!("Send mail to {to}");

    if cfg!(debug_assertions) {
        error!("Mail sending is disabled for debug builds");
        return Err(Error::Network);
    }

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
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(host)?
        .credentials(Credentials::new(from.to_string(), password.to_string()))
        .timeout(Some(Duration::from_secs(1)))
        .build();

    // Send the email
    mailer.send(email).await?;
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
    #[tokio::test]
    #[ignore]
    async fn send_mail() {
        crate::logging();
        super::send(
            &std::env::var("SBV_MAIL_HOST").unwrap(),
            &std::env::var("SBV_MAIL_PASSWORD").unwrap(),
            &std::env::var("SBV_MAIL_FROM").unwrap(),
            &std::env::var("SBV_MAIL_TO").unwrap(),
            "Test Mail 🚧",
            "Test Content 🚧",
        )
        .await
        .unwrap();
    }
}
