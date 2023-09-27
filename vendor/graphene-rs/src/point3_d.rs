// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Point3D;
use crate::Vec3;
use glib::translate::*;
use std::fmt;

impl Point3D {
    #[doc(alias = "graphene_point3d_init")]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut p = Self::uninitialized();
            ffi::graphene_point3d_init(p.to_glib_none_mut().0, x, y, z);
            p
        }
    }

    #[doc(alias = "graphene_point3d_init_from_vec3")]
    #[doc(alias = "init_from_vec3")]
    pub fn from_vec3(v: &Vec3) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut p = Self::uninitialized();
            ffi::graphene_point3d_init_from_vec3(p.to_glib_none_mut().0, v.to_glib_none().0);
            p
        }
    }

    pub fn x(&self) -> f32 {
        self.inner.x
    }

    pub fn set_x(&mut self, x: f32) {
        self.inner.x = x;
    }

    pub fn y(&self) -> f32 {
        self.inner.y
    }

    pub fn set_y(&mut self, y: f32) {
        self.inner.y = y;
    }

    pub fn z(&self) -> f32 {
        self.inner.z
    }

    pub fn set_z(&mut self, z: f32) {
        self.inner.z = z;
    }
}

impl fmt::Debug for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point3D")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("z", &self.z())
            .finish()
    }
}
