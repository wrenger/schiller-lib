use std::thread;

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
#[derive(NativeClass, Debug, Default)]
#[inherit(Reference)]
#[register_with(Self::register)]
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

    /// Internal sender thread.
    worker: Option<thread::JoinHandle<()>>,
}

#[methods]
impl Mailer {
    fn new(_owner: &Reference) -> Self {
        Mailer::default()
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .signal("done")
            .with_param_default("result", Dictionary::default().to_variant())
            .done();
    }

    /// Asynchronously sends a mail to the given user with the specified `title` and `body`.
    /// The `done` signal is emitted when the sending has been completed.
    ///
    /// The method cannot be called again before the `done` signal has been emitted!
    #[export]
    fn send(
        &mut self,
        owner: TRef<Reference>,
        to: String,
        title: String,
        body: String,
    ) -> api::Result<()> {
        // Sending is in progress.
        if self.worker.is_some() {
            return Err(api::Error::Logic);
        }

        let owner = owner.claim();
        let host = self.host.clone();
        let password = self.password.clone();
        let from = self.from.clone();
        self.worker = Some(thread::spawn(move || {
            let result = mail::send(&host, &password, &from, &to, &title, &body);
            // Signal that we are finished.
            unsafe {
                let owner = owner.assume_safe();
                owner.call_deferred("_send_done", &[result.to_variant()]);
            }
        }));

        Ok(())
    }

    #[export]
    fn _send_done(&mut self, owner: TRef<Reference>, result: api::Result<()>) {
        if let Some(thread) = self.worker.take() {
            if let Err(e) = thread.join() {
                error!("Error dropping mailer worker: {e:?}");
            }
        }
        // emit signal after this method is finished
        // -> no reentry while mutably borrowing self
        unsafe {
            owner.call_deferred("emit_signal", &["done".to_variant(), result.to_variant()]);
        }
    }
}

impl Drop for Mailer {
    fn drop(&mut self) {
        if let Some(thread) = self.worker.take() {
            if let Err(e) = thread.join() {
                error!("Error dropping mailer worker: {e:?}");
            }
        }
    }
}
