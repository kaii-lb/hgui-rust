use crate::PreferencesRow;
use glib::subclass::prelude::*;
use gtk::subclass::list_box_row::ListBoxRowImpl;

pub trait PreferencesRowImpl: ListBoxRowImpl {}

unsafe impl<T: PreferencesRowImpl> IsSubclassable<T> for PreferencesRow {}
