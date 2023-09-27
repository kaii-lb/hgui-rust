use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use crate::subclass::prelude::PreferencesRowImpl;
use crate::ActionRow;

pub trait ActionRowImpl: PreferencesRowImpl {
    fn activate(&self) {
        ActionRowImplExt::parent_activate(self)
    }
}

pub trait ActionRowImplExt: ObjectSubclass {
    fn parent_activate(&self);
}

impl<T: ActionRowImpl> ActionRowImplExt for T {
    fn parent_activate(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::AdwActionRowClass;
            if let Some(f) = (*parent_class).activate {
                f(self
                    .instance()
                    .unsafe_cast_ref::<ActionRow>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

unsafe impl<T: ActionRowImpl> IsSubclassable<T> for ActionRow {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.activate = Some(row_activate::<T>);
    }
}

unsafe extern "C" fn row_activate<T: ActionRowImpl>(ptr: *mut ffi::AdwActionRow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    ActionRowImpl::activate(imp)
}
