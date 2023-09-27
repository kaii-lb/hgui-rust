use super::preferences_row::PreferencesRowImpl;
use crate::ExpanderRow;
use glib::subclass::prelude::*;

pub trait ExpanderRowImpl: PreferencesRowImpl {}

unsafe impl<T: ExpanderRowImpl> IsSubclassable<T> for ExpanderRow {}
