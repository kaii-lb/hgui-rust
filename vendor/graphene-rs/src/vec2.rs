// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Vec2;
use glib::translate::*;
use std::fmt;

impl Vec2 {
    #[doc(alias = "graphene_vec2_init")]
    pub fn new(x: f32, y: f32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Self::uninitialized();
            ffi::graphene_vec2_init(vec.to_glib_none_mut().0, x, y);
            vec
        }
    }

    #[doc(alias = "graphene_vec2_init_from_float")]
    #[doc(alias = "init_from_float")]
    pub fn from_float(src: [f32; 2]) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Self::uninitialized();
            ffi::graphene_vec2_init_from_float(vec.to_glib_none_mut().0, src.as_ptr() as *const _);
            vec
        }
    }

    #[doc(alias = "graphene_vec2_to_float")]
    pub fn to_float(&self) -> [f32; 2] {
        unsafe {
            let mut out = std::mem::MaybeUninit::uninit();
            ffi::graphene_vec2_to_float(self.to_glib_none().0, out.as_mut_ptr());
            out.assume_init()
        }
    }
}

impl fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vec2")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}
