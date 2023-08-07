pub mod code;

use glib::translate::FromGlibPtrFull;
use gtk::glib;
use libc::{c_int, EXIT_SUCCESS};

static mut BUILDER: Option<gtk::Builder> = None;

#[no_mangle]
extern "C" fn run() -> c_int {
    code::main().into()
}

#[no_mangle]
extern "C" fn set_builder(builder_ptr: *mut gtk::ffi::GtkBuilder) -> c_int {
    unsafe {
        let gtk_builder = gtk::Builder::from_glib_full(builder_ptr);
        BUILDER = Some(gtk_builder);
    }
    EXIT_SUCCESS
}

pub fn builder() -> &'static gtk::Builder {
    unsafe {
        BUILDER
            .as_ref()
            .expect("Builder instance should already be initialized.")
    }
}
