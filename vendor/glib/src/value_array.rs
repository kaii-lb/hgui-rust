// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use crate::Value;
use std::cmp::Ordering;
use std::ops;
use std::slice;

wrapper! {
    #[derive(Debug)]
    #[doc(alias = "GValueArray")]
    pub struct ValueArray(Boxed<gobject_ffi::GValueArray>);

    match fn {
        copy => |ptr| gobject_ffi::g_value_array_copy(mut_override(ptr)),
        free => |ptr| gobject_ffi::g_value_array_free(ptr),
        type_ => || gobject_ffi::g_value_array_get_type(),
    }
}

impl ValueArray {
    #[doc(alias = "g_value_array_new")]
    pub fn new(n_prealloced: u32) -> ValueArray {
        unsafe { from_glib_full(gobject_ffi::g_value_array_new(n_prealloced)) }
    }

    #[doc(alias = "g_value_array_append")]
    pub fn append(&mut self, value: &Value) {
        let value = value.to_glib_none();
        unsafe {
            gobject_ffi::g_value_array_append(self.to_glib_none_mut().0, value.0);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        let value = self.to_glib_none();
        value.1.n_values as usize
    }

    #[doc(alias = "get_nth")]
    #[doc(alias = "g_value_array_get_nth")]
    pub fn nth(&self, index_: u32) -> Option<Value> {
        unsafe {
            from_glib_none(gobject_ffi::g_value_array_get_nth(
                mut_override(self.to_glib_none().0),
                index_,
            ))
        }
    }

    #[doc(alias = "g_value_array_insert")]
    pub fn insert(&mut self, index_: u32, value: &Value) {
        let value = value.to_glib_none();
        unsafe {
            gobject_ffi::g_value_array_insert(self.to_glib_none_mut().0, index_, value.0);
        }
    }

    #[doc(alias = "g_value_array_prepend")]
    pub fn prepend(&mut self, value: &Value) {
        let value = value.to_glib_none();
        unsafe {
            gobject_ffi::g_value_array_prepend(self.to_glib_none_mut().0, value.0);
        }
    }

    #[doc(alias = "g_value_array_remove")]
    pub fn remove(&mut self, index_: u32) {
        unsafe {
            gobject_ffi::g_value_array_remove(self.to_glib_none_mut().0, index_);
        }
    }

    #[doc(alias = "g_value_array_sort_with_data")]
    pub fn sort_with_data<F: FnMut(&Value, &Value) -> Ordering>(&mut self, compare_func: F) {
        unsafe extern "C" fn compare_func_trampoline(
            a: ffi::gconstpointer,
            b: ffi::gconstpointer,
            func: ffi::gpointer,
        ) -> i32 {
            let func = func as *mut &mut (dyn FnMut(&Value, &Value) -> Ordering);

            let a = &*(a as *const Value);
            let b = &*(b as *const Value);

            (*func)(a, b).into_glib()
        }
        unsafe {
            let mut func = compare_func;
            let func_obj: &mut (dyn FnMut(&Value, &Value) -> Ordering) = &mut func;
            let func_ptr =
                &func_obj as *const &mut (dyn FnMut(&Value, &Value) -> Ordering) as ffi::gpointer;

            gobject_ffi::g_value_array_sort_with_data(
                self.to_glib_none_mut().0,
                Some(compare_func_trampoline),
                func_ptr,
            );
        }
    }
}

impl ops::Deref for ValueArray {
    type Target = [Value];

    fn deref(&self) -> &[Value] {
        if self.is_empty() {
            return &[];
        }

        unsafe {
            slice::from_raw_parts(
                (*self.to_glib_none().0).values as *const Value,
                (*self.to_glib_none().0).n_values as usize,
            )
        }
    }
}

impl ops::DerefMut for ValueArray {
    fn deref_mut(&mut self) -> &mut [Value] {
        if self.is_empty() {
            return &mut [];
        }

        unsafe {
            slice::from_raw_parts_mut(
                (*self.to_glib_none().0).values as *mut Value,
                (*self.to_glib_none().0).n_values as usize,
            )
        }
    }
}
