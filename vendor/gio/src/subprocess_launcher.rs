// Take a look at the license at the top of the repository in the LICENSE file.

use crate::SubprocessLauncher;
#[cfg(any(unix, feature = "dox"))]
#[cfg(any(unix, feature = "dox"))]
use glib::translate::*;
#[cfg(any(unix, all(feature = "dox", unix)))]
use std::os::unix::io::IntoRawFd;

#[cfg(all(feature = "dox", not(unix)))]
pub trait IntoRawFd: Sized {
    fn into_raw_fd(self) -> i32 {
        0
    }
}

impl SubprocessLauncher {
    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_fd")]
    pub fn take_fd(&self, source_fd: impl IntoRawFd, target_fd: impl IntoRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_fd(
                self.to_glib_none().0,
                source_fd.into_raw_fd(),
                target_fd.into_raw_fd(),
            );
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_stderr_fd")]
    pub fn take_stderr_fd(&self, fd: impl IntoRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_stderr_fd(self.to_glib_none().0, fd.into_raw_fd());
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_stdin_fd")]
    pub fn take_stdin_fd(&self, fd: impl IntoRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdin_fd(self.to_glib_none().0, fd.into_raw_fd());
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_stdout_fd")]
    pub fn take_stdout_fd(&self, fd: impl IntoRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdout_fd(self.to_glib_none().0, fd.into_raw_fd());
        }
    }
}
