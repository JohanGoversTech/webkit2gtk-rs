// This file was generated by gir (d50d839) from gir-files (469db10)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct PermissionRequest(Object<ffi::WebKitPermissionRequest, ffi::WebKitPermissionRequestIface>);

    match fn {
        get_type => || ffi::webkit_permission_request_get_type(),
    }
}

pub trait PermissionRequestExt {
    fn allow(&self);

    fn deny(&self);
}

impl<O: IsA<PermissionRequest>> PermissionRequestExt for O {
    fn allow(&self) {
        unsafe {
            ffi::webkit_permission_request_allow(self.to_glib_none().0);
        }
    }

    fn deny(&self) {
        unsafe {
            ffi::webkit_permission_request_deny(self.to_glib_none().0);
        }
    }
}
