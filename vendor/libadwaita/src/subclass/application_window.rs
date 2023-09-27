use crate::ApplicationWindow;
use gtk::subclass::prelude::*;

pub trait AdwApplicationWindowImpl: ApplicationWindowImpl {}

unsafe impl<T: AdwApplicationWindowImpl> IsSubclassable<T> for ApplicationWindow {}
