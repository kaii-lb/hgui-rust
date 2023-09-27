use crate::PreferencesPage;
use glib::subclass::prelude::*;
use gtk::subclass::widget::WidgetImpl;

pub trait PreferencesPageImpl: WidgetImpl {}

unsafe impl<T: PreferencesPageImpl> IsSubclassable<T> for PreferencesPage {}
