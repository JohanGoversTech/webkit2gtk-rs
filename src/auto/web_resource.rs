// This file was generated by gir (d50d839) from gir-files (469db10)
// DO NOT EDIT

use Error;
use URIRequest;
use URIResponse;
use ffi;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use gio;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use gio_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct WebResource(Object<ffi::WebKitWebResource, ffi::WebKitWebResourceClass>);

    match fn {
        get_type => || ffi::webkit_web_resource_get_type(),
    }
}

pub trait WebResourceExt {
    //fn get_data<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R);

    fn get_response(&self) -> Option<URIResponse>;

    fn get_uri(&self) -> Option<String>;

    fn connect_failed<F: Fn(&Self, &Error) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_failed_with_tls_errors<F: Fn(&Self, &gio::TlsCertificate, gio::TlsCertificateFlags) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_received_data<F: Fn(&Self, u64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_sent_request<F: Fn(&Self, &URIRequest, &URIResponse) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebResource> + IsA<glib::object::Object>> WebResourceExt for O {
    //fn get_data<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::webkit_web_resource_get_data() }
    //}

    fn get_response(&self) -> Option<URIResponse> {
        unsafe {
            from_glib_none(ffi::webkit_web_resource_get_response(self.to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_web_resource_get_uri(self.to_glib_none().0))
        }
    }

    fn connect_failed<F: Fn(&Self, &Error) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Error) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "failed",
                transmute(failed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_failed_with_tls_errors<F: Fn(&Self, &gio::TlsCertificate, gio::TlsCertificateFlags) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gio::TlsCertificate, gio::TlsCertificateFlags) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "failed-with-tls-errors",
                transmute(failed_with_tls_errors_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "finished",
                transmute(finished_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_received_data<F: Fn(&Self, u64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, u64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "received-data",
                transmute(received_data_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_sent_request<F: Fn(&Self, &URIRequest, &URIResponse) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &URIRequest, &URIResponse) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "sent-request",
                transmute(sent_request_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::response",
                transmute(notify_response_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::uri",
                transmute(notify_uri_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn failed_trampoline<P>(this: *mut ffi::WebKitWebResource, error: *mut glib_ffi::GError, f: glib_ffi::gpointer)
where P: IsA<WebResource> {
    callback_guard!();
    let f: &&(Fn(&P, &Error) + 'static) = transmute(f);
    f(&WebResource::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(error))
}

#[cfg(any(feature = "v2_8", feature = "dox"))]
unsafe extern "C" fn failed_with_tls_errors_trampoline<P>(this: *mut ffi::WebKitWebResource, certificate: *mut gio_ffi::GTlsCertificate, errors: gio_ffi::GTlsCertificateFlags, f: glib_ffi::gpointer)
where P: IsA<WebResource> {
    callback_guard!();
    let f: &&(Fn(&P, &gio::TlsCertificate, gio::TlsCertificateFlags) + 'static) = transmute(f);
    f(&WebResource::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(certificate), from_glib(errors))
}

unsafe extern "C" fn finished_trampoline<P>(this: *mut ffi::WebKitWebResource, f: glib_ffi::gpointer)
where P: IsA<WebResource> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebResource::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn received_data_trampoline<P>(this: *mut ffi::WebKitWebResource, data_length: u64, f: glib_ffi::gpointer)
where P: IsA<WebResource> {
    callback_guard!();
    let f: &&(Fn(&P, u64) + 'static) = transmute(f);
    f(&WebResource::from_glib_borrow(this).downcast_unchecked(), data_length)
}

unsafe extern "C" fn sent_request_trampoline<P>(this: *mut ffi::WebKitWebResource, request: *mut ffi::WebKitURIRequest, redirected_response: *mut ffi::WebKitURIResponse, f: glib_ffi::gpointer)
where P: IsA<WebResource> {
    callback_guard!();
    let f: &&(Fn(&P, &URIRequest, &URIResponse) + 'static) = transmute(f);
    f(&WebResource::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(request), &from_glib_borrow(redirected_response))
}

unsafe extern "C" fn notify_response_trampoline<P>(this: *mut ffi::WebKitWebResource, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebResource> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebResource::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_uri_trampoline<P>(this: *mut ffi::WebKitWebResource, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebResource> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebResource::from_glib_borrow(this).downcast_unchecked())
}
