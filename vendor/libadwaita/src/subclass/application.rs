use crate::Application;

use gtk::subclass::prelude::*;

pub trait AdwApplicationImpl: GtkApplicationImpl {}

unsafe impl<T: AdwApplicationImpl> IsSubclassable<T> for Application {}
