pub mod code;
use glib::translate::FromGlibPtrFull;
use gtk::glib;
use libc::c_int;

#[no_mangle]
extern "C" fn run(builder_ptr: *mut gtk::ffi::GtkBuilder) -> c_int {
    let builder = unsafe { gtk::Builder::from_glib_full(builder_ptr) };
    code::main(builder).into()
}
