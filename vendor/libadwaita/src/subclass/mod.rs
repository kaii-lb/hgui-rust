pub mod action_row;
pub mod application;
pub mod application_window;
pub mod bin;
pub mod combo_row;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
pub mod entry_row;
pub mod expander_row;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
mod message_dialog;
pub mod preferences_group;
pub mod preferences_page;
pub mod preferences_row;
pub mod preferences_window;
pub mod swipeable;
pub mod window;

pub mod prelude {
    pub use super::action_row::ActionRowImpl;
    pub use super::application::AdwApplicationImpl;
    pub use super::application_window::AdwApplicationWindowImpl;
    pub use super::bin::BinImpl;
    pub use super::combo_row::ComboRowImpl;
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    pub use super::entry_row::EntryRowImpl;
    pub use super::expander_row::ExpanderRowImpl;
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    pub use super::message_dialog::{MessageDialogImpl, MessageDialogImplExt};
    pub use super::preferences_group::PreferencesGroupImpl;
    pub use super::preferences_page::PreferencesPageImpl;
    pub use super::preferences_row::PreferencesRowImpl;
    pub use super::preferences_window::PreferencesWindowImpl;
    pub use super::swipeable::SwipeableImpl;
    pub use super::window::AdwWindowImpl;
    pub use gio::subclass::prelude::*;
    pub use glib::subclass::prelude::*;
    pub use gtk::subclass::prelude::*;
}
