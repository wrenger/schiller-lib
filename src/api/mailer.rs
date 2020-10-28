use gdnative::prelude::*;

use crate::api;
use crate::mail;

/// The Mail wrapper "class"
#[derive(NativeClass, Default, Debug)]
#[inherit(Reference)]
pub struct Mailer {
    #[property]
    from: String,
    #[property]
    host: String,
    #[property]
    password: String,
}

#[methods]
impl Mailer {
    fn new(_owner: &Reference) -> Self {
        Mailer::default()
    }

    #[export]
    fn send(&self, _owner: &Reference, to: String, title: String, body: String) -> api::Result<()> {
        mail::send(&self.host, &self.password, &self.from, &to, &title, &body)
    }
}
