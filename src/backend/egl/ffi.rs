#![allow(missing_docs)]

use nix::libc::{c_long, c_uint, c_void, int32_t, uint64_t};

pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
pub type khronos_uint64_t = uint64_t;
pub type khronos_ssize_t = c_long;
pub type EGLint = int32_t;
pub type EGLNativeDisplayType = NativeDisplayType;
pub type EGLNativePixmapType = NativePixmapType;
pub type EGLNativeWindowType = NativeWindowType;
pub type NativeDisplayType = *const c_void;
pub type NativePixmapType = *const c_void;
pub type NativeWindowType = *const c_void;

#[allow(clippy::all, rust_2018_idioms)]
pub mod egl {
    use super::*;
    use libloading::Library;
    use std::sync::Once;

    lazy_static! {
        pub static ref LIB: Library = { Library::new("libEGL.so.1").expect("Failed to load LibEGL") };
    }

    pub static LOAD: Once = Once::new();

    include!(concat!(env!("OUT_DIR"), "/egl_bindings.rs"));

    /*
     * `gl_generator` cannot generate bindings for the `EGL_WL_bind_wayland_display` extension.
     *  Lets do it ourselves...
     */

    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BindWaylandDisplayWL(
        dpy: types::EGLDisplay,
        display: *mut __gl_imports::raw::c_void,
    ) -> types::EGLBoolean {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::EGLDisplay, *mut __gl_imports::raw::c_void) -> types::EGLBoolean,
        >(wayland_storage::BindWaylandDisplayWL.f)(dpy, display)
    }

    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UnbindWaylandDisplayWL(
        dpy: types::EGLDisplay,
        display: *mut __gl_imports::raw::c_void,
    ) -> types::EGLBoolean {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::EGLDisplay, *mut __gl_imports::raw::c_void) -> types::EGLBoolean,
        >(wayland_storage::UnbindWaylandDisplayWL.f)(dpy, display)
    }

    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn QueryWaylandBufferWL(
        dpy: types::EGLDisplay,
        buffer: *mut __gl_imports::raw::c_void,
        attribute: types::EGLint,
        value: *mut types::EGLint,
    ) -> types::EGLBoolean {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::EGLDisplay,
                *mut __gl_imports::raw::c_void,
                types::EGLint,
                *mut types::EGLint,
            ) -> types::EGLBoolean,
        >(wayland_storage::QueryWaylandBufferWL.f)(dpy, buffer, attribute, value)
    }

    mod wayland_storage {
        use super::{FnPtr, __gl_imports::raw};
        pub static mut BindWaylandDisplayWL: FnPtr = FnPtr {
            f: super::missing_fn_panic as *const raw::c_void,
            is_loaded: false,
        };
        pub static mut UnbindWaylandDisplayWL: FnPtr = FnPtr {
            f: super::missing_fn_panic as *const raw::c_void,
            is_loaded: false,
        };
        pub static mut QueryWaylandBufferWL: FnPtr = FnPtr {
            f: super::missing_fn_panic as *const raw::c_void,
            is_loaded: false,
        };
    }

    #[allow(non_snake_case)]
    pub mod BindWaylandDisplayWL {
        use super::{FnPtr, __gl_imports::raw, metaloadfn, wayland_storage};

        #[inline]
        #[allow(dead_code)]
        pub fn is_loaded() -> bool {
            unsafe { wayland_storage::BindWaylandDisplayWL.is_loaded }
        }

        #[allow(dead_code)]
        pub fn load_with<F>(mut loadfn: F)
        where
            F: FnMut(&str) -> *const raw::c_void,
        {
            unsafe {
                wayland_storage::BindWaylandDisplayWL =
                    FnPtr::new(metaloadfn(&mut loadfn, "eglBindWaylandDisplayWL", &[]))
            }
        }
    }

    #[allow(non_snake_case)]
    pub mod UnbindWaylandDisplayWL {
        use super::{FnPtr, __gl_imports::raw, metaloadfn, wayland_storage};

        #[inline]
        #[allow(dead_code)]
        pub fn is_loaded() -> bool {
            unsafe { wayland_storage::UnbindWaylandDisplayWL.is_loaded }
        }

        #[allow(dead_code)]
        pub fn load_with<F>(mut loadfn: F)
        where
            F: FnMut(&str) -> *const raw::c_void,
        {
            unsafe {
                wayland_storage::UnbindWaylandDisplayWL =
                    FnPtr::new(metaloadfn(&mut loadfn, "eglUnbindWaylandDisplayWL", &[]))
            }
        }
    }

    #[allow(non_snake_case)]
    pub mod QueryWaylandBufferWL {
        use super::{FnPtr, __gl_imports::raw, metaloadfn, wayland_storage};

        #[inline]
        #[allow(dead_code)]
        pub fn is_loaded() -> bool {
            unsafe { wayland_storage::QueryWaylandBufferWL.is_loaded }
        }

        #[allow(dead_code)]
        pub fn load_with<F>(mut loadfn: F)
        where
            F: FnMut(&str) -> *const raw::c_void,
        {
            unsafe {
                wayland_storage::QueryWaylandBufferWL =
                    FnPtr::new(metaloadfn(&mut loadfn, "eglQueryWaylandBufferWL", &[]))
            }
        }
    }

    // Accepted as <target> in eglCreateImageKHR
    pub const WAYLAND_BUFFER_WL: c_uint = 0x31D5;
    // Accepted in the <attrib_list> parameter of eglCreateImageKHR:
    pub const WAYLAND_PLANE_WL: c_uint = 0x31D6;
    // Possible values for EGL_TEXTURE_FORMAT:
    pub const TEXTURE_Y_U_V_WL: int32_t = 0x31D7;
    pub const TEXTURE_Y_UV_WL: int32_t = 0x31D8;
    pub const TEXTURE_Y_XUXV_WL: int32_t = 0x31D9;
    pub const TEXTURE_EXTERNAL_WL: int32_t = 0x31DA;
    // Accepted in the <attribute> parameter of eglQueryWaylandBufferWL:
    pub const EGL_TEXTURE_FORMAT: int32_t = 0x3080;
    pub const WAYLAND_Y_INVERTED_WL: int32_t = 0x31DB;
}
