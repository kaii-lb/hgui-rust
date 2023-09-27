#[doc(hidden)]
pub use gtk::prelude::*;

pub use crate::auto::traits::*;

#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
pub use crate::message_dialog::MessageDialogExtManual;
