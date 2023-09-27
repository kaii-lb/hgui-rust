// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Euler;
use crate::Matrix;
use crate::Quaternion;
use crate::Vec3;
use crate::Vec4;
use glib::translate::*;
use std::fmt;

impl Quaternion {
    #[doc(alias = "graphene_quaternion_init")]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Self::uninitialized();
            ffi::graphene_quaternion_init(quat.to_glib_none_mut().0, x, y, z, w);
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_angle_vec3")]
    #[doc(alias = "init_from_angle_vec3")]
    pub fn from_angle_vec3(angle: f32, axis: &Vec3) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Self::uninitialized();
            ffi::graphene_quaternion_init_from_angle_vec3(
                quat.to_glib_none_mut().0,
                angle,
                axis.to_glib_none().0,
            );
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_angles")]
    #[doc(alias = "init_from_angles")]
    pub fn from_angles(deg_x: f32, deg_y: f32, deg_z: f32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Self::uninitialized();
            ffi::graphene_quaternion_init_from_angles(
                quat.to_glib_none_mut().0,
                deg_x,
                deg_y,
                deg_z,
            );
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_euler")]
    #[doc(alias = "init_from_euler")]
    pub fn from_euler(e: &Euler) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Self::uninitialized();
            ffi::graphene_quaternion_init_from_euler(quat.to_glib_none_mut().0, e.to_glib_none().0);
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_matrix")]
    #[doc(alias = "init_from_matrix")]
    pub fn from_matrix(m: &Matrix) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Self::uninitialized();
            ffi::graphene_quaternion_init_from_matrix(
                quat.to_glib_none_mut().0,
                m.to_glib_none().0,
            );
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_radians")]
    #[doc(alias = "init_from_radians")]
    pub fn from_radians(rad_x: f32, rad_y: f32, rad_z: f32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Self::uninitialized();
            ffi::graphene_quaternion_init_from_radians(
                quat.to_glib_none_mut().0,
                rad_x,
                rad_y,
                rad_z,
            );
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_vec4")]
    #[doc(alias = "init_from_vec4")]
    pub fn from_vec4(src: &Vec4) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Self::uninitialized();
            ffi::graphene_quaternion_init_from_vec4(
                quat.to_glib_none_mut().0,
                src.to_glib_none().0,
            );
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_identity")]
    #[doc(alias = "init_identity")]
    pub fn new_identity() -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Self::uninitialized();
            ffi::graphene_quaternion_init_identity(quat.to_glib_none_mut().0);
            quat
        }
    }

    pub fn x(&self) -> f32 {
        self.inner.x
    }

    pub fn y(&self) -> f32 {
        self.inner.y
    }

    pub fn z(&self) -> f32 {
        self.inner.z
    }

    pub fn w(&self) -> f32 {
        self.inner.w
    }
}

impl fmt::Debug for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Quaternion")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("z", &self.z())
            .field("w", &self.w())
            .finish()
    }
}
