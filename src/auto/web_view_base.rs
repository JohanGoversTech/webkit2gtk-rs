// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct WebViewBase(Object<ffi::WebKitWebViewBase>): [
        gtk::Container => gtk_ffi::GtkContainer,
        gtk::Widget => gtk_ffi::GtkWidget,
    ];

    match fn {
        get_type => || ffi::webkit_web_view_base_get_type(),
    }
}

impl WebViewBase {}
