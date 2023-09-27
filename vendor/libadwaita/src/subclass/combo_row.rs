use super::action_row::ActionRowImpl;
use crate::ComboRow;
use glib::subclass::prelude::*;

pub trait ComboRowImpl: ActionRowImpl {}

unsafe impl<T: ComboRowImpl> IsSubclassable<T> for ComboRow {}
