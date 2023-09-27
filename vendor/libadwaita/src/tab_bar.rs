// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{TabBar, TabPage};
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;

impl TabBar {
    #[doc(alias = "adw_tab_bar_setup_extra_drop_target")]
    pub fn setup_extra_drop_target(&self, actions: gdk::DragAction, types: &[glib::Type]) {
        unsafe {
            ffi::adw_tab_bar_setup_extra_drop_target(
                self.to_glib_none().0,
                actions.into_glib(),
                types.to_glib_none().0,
                types.len() as usize,
            )
        }
    }

    #[doc(alias = "extra-drag-drop")]
    pub fn connect_extra_drag_drop<F: Fn(&TabBar, &TabPage, &glib::Value) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn extra_drag_drop_trampoline<
            F: Fn(&TabBar, &TabPage, &glib::Value) -> bool + 'static,
        >(
            this: *mut ffi::AdwTabBar,
            page: *mut ffi::AdwTabPage,
            value: *mut glib::gobject_ffi::GValue,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(page),
                &*(value as *const glib::Value),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"extra-drag-drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    extra_drag_drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
