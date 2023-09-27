use crate::Bin;
use glib::subclass::prelude::*;
use gtk::subclass::prelude::WidgetImpl;

pub trait BinImpl: WidgetImpl {}

unsafe impl<T: BinImpl> IsSubclassable<T> for Bin {}
