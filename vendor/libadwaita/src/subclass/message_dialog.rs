use crate::MessageDialog;
use glib::translate::*;
use glib::Cast;
use gtk::subclass::prelude::*;

pub trait MessageDialogImpl: gtk::subclass::prelude::WindowImpl {
    fn response(&self, response: &str) {
        self.parent_response(response)
    }
}

pub trait MessageDialogImplExt: ObjectSubclass {
    fn parent_response(&self, response: &str);
}

impl<T: MessageDialogImpl> MessageDialogImplExt for T {
    fn parent_response(&self, response: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::AdwMessageDialogClass;
            if let Some(f) = (*parent_class).response {
                f(
                    self.instance()
                        .unsafe_cast_ref::<MessageDialog>()
                        .to_glib_none()
                        .0,
                    response.to_glib_none().0,
                )
            }
        }
    }
}

unsafe impl<T: MessageDialogImpl> IsSubclassable<T> for MessageDialog {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.response = Some(message_dialog_response::<T>);
    }
}

unsafe extern "C" fn message_dialog_response<T: MessageDialogImpl>(
    ptr: *mut ffi::AdwMessageDialog,
    response: *const libc::c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let response: Borrowed<glib::GString> = from_glib_borrow(response);

    imp.response(response.as_ref())
}
