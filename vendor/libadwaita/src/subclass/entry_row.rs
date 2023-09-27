use glib::subclass::prelude::*;

use crate::subclass::prelude::PreferencesRowImpl;
use crate::EntryRow;

pub trait EntryRowImpl: PreferencesRowImpl {}

unsafe impl<T: EntryRowImpl> IsSubclassable<T> for EntryRow {}
