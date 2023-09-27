use glib::object::IsA;
use glib::translate::*;
use std::cell::{Cell, RefCell};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use crate::prelude::*;
use crate::MessageDialog;

pub trait MessageDialogExtManual: 'static {
    #[doc(alias = "adw_message_dialog_get_response_label")]
    #[doc(alias = "get_response_label")]
    fn response_label(&self, response: &str) -> glib::GString;

    #[doc(alias = "adw_message_dialog_add_responses")]
    fn add_responses(&self, ids_and_labels: &[(&str, &str)]);

    // rustdoc-stripper-ignore-next
    /// Shows the dialog and returns a `Future` that resolves to the
    /// `ResponseType` on response.
    ///
    /// ```no_run
    /// use libadwaita::prelude::*;
    ///
    /// # async fn run() {
    /// let dialog = libadwaita::MessageDialog::builder()
    ///    .body("What is your answer?")
    ///    .build();
    ///
    /// dialog.add_responses(&[("ok", "Ok"), ("cancel", "Cancel")]);
    ///
    /// let answer = dialog.run_future(None).await;
    /// dialog.close();
    /// println!("Answer: {answer}");
    /// # }
    /// ```
    fn run_future<'a>(&'a self) -> Pin<Box<dyn Future<Output = String> + 'a>>;

    // rustdoc-stripper-ignore-next
    /// Shows the dialog and calls the callback when a response has been received.
    ///
    /// **Important**: this function isn't blocking.
    ///
    /// ```no_run
    /// use libadwaita::prelude::*;
    ///
    /// let dialog = libadwaita::MessageDialog::builder()
    ///    .body("What is your answer?")
    ///    .build();
    ///
    /// dialog.add_responses(&[("ok", "Ok"), ("cancel", "Cancel")]);
    ///
    /// dialog.run_async(None, |obj, answer| {
    ///     obj.close();
    ///     println!("Answer: {:?}", answer);
    /// });
    /// ```
    fn run_async<F: FnOnce(&Self, &str) + 'static>(&self, detail: Option<&str>, f: F);
}

impl<O: IsA<MessageDialog>> MessageDialogExtManual for O {
    #[doc(alias = "adw_message_dialog_get_response_label")]
    #[doc(alias = "get_response_label")]
    fn response_label(&self, response: &str) -> glib::GString {
        assert!(self.as_ref().has_response(response));

        unsafe {
            from_glib_none(ffi::adw_message_dialog_get_response_label(
                self.as_ref().to_glib_none().0,
                response.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_message_dialog_add_responses")]
    fn add_responses(&self, ids_and_labels: &[(&str, &str)]) {
        ids_and_labels.iter().for_each(|(id, label)| {
            self.add_response(id, label);
        });
    }

    fn run_future<'a>(&'a self) -> Pin<Box<dyn Future<Output = String> + 'a>> {
        Box::pin(async move {
            let (sender, receiver) = futures_channel::oneshot::channel::<String>();

            let sender = Cell::new(Some(sender));

            let response_handler = self.connect_response(None, move |_, response| {
                if let Some(m) = sender.replace(None) {
                    let _result = m.send(response.to_string());
                }
            });

            self.as_ref().present();

            if let Ok(response) = receiver.await {
                self.disconnect(response_handler);
                response
            } else {
                String::new()
            }
        })
    }

    fn run_async<F: FnOnce(&Self, &str) + 'static>(&self, detail: Option<&str>, f: F) {
        let response_handler = Rc::new(RefCell::new(None));
        let response_handler_clone = response_handler.clone();
        let f = RefCell::new(Some(f));
        *response_handler.borrow_mut() = Some(self.connect_response(detail, move |s, response| {
            if let Some(handler) = response_handler_clone.borrow_mut().take() {
                s.disconnect(handler);
            }
            (*f.borrow_mut()).take().expect("cannot get callback")(s, response);
        }));
        self.as_ref().present();
    }
}
