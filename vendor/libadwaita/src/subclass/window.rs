use crate::Window;
use gtk::subclass::prelude::*;

pub trait AdwWindowImpl: WindowImpl {}

unsafe impl<T: AdwWindowImpl> IsSubclassable<T> for Window {}
