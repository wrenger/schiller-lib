use gdnative::prelude::*;

use crate::api;
use crate::mail;

/// The mail class provides method for sending mail messages to users of the
/// library.
///
/// Mail addresses are constructed in the form of `<account>@<host>`.
///
/// It is assumed that the sender and the recipients have the same mail
/// server and that their username name matches the account name on the
/// mail server.
///
/// The credentials are stored in the database.
#[derive(NativeClass, Default, Debug)]
#[inherit(Reference)]
pub struct Mailer {
    /// Username of the sender.
    #[property]
    from: String,
    /// Host name of the mail server.
    #[property]
    host: String,
    /// Password of the sender.
    #[property]
    password: String,
}

#[methods]
impl Mailer {
    fn new(_owner: &Reference) -> Self {
        Mailer::default()
    }

    /// Sends a mail to the given user with the specified `title` and `body`.
    #[export]
    fn send(&self, _owner: &Reference, to: String, title: String, body: String) -> api::Result<()> {
        mail::send(&self.host, &self.password, &self.from, &to, &title, &body)
    }
}
