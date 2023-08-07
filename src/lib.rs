mod code;
pub(crate) mod workbench;

use glib::translate::FromGlibPtrFull;
use gtk::glib;
use libc::{c_int, EXIT_SUCCESS};

static mut BUILDER: Option<gtk::Builder> = None;
static mut WINDOW: Option<gtk::Window> = None;

#[no_mangle]
extern "C" fn execute() -> c_int {
    code::main().into()
}

#[no_mangle]
extern "C" fn set_builder(builder_ptr: *mut gtk::ffi::GtkBuilder) -> c_int {
    unsafe {
        let builder = gtk::Builder::from_glib_full(builder_ptr);
        BUILDER = Some(builder);
    }
    EXIT_SUCCESS
}

#[no_mangle]
extern "C" fn set_window(window_ptr: *mut gtk::ffi::GtkWindow) -> c_int {
    unsafe {
        let window = gtk::Window::from_glib_full(window_ptr);
        WINDOW = Some(window);
    }
    EXIT_SUCCESS
}
