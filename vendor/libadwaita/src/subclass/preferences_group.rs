use crate::PreferencesGroup;
use glib::subclass::prelude::*;
use gtk::subclass::widget::WidgetImpl;

pub trait PreferencesGroupImpl: WidgetImpl {}

unsafe impl<T: PreferencesGroupImpl> IsSubclassable<T> for PreferencesGroup {}
