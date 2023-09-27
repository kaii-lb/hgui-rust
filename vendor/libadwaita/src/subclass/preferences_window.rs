use super::window::AdwWindowImpl;
use crate::PreferencesWindow;
use glib::subclass::prelude::*;

pub trait PreferencesWindowImpl: AdwWindowImpl {}

unsafe impl<T: PreferencesWindowImpl> IsSubclassable<T> for PreferencesWindow {}
