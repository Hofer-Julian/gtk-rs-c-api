// call_so.rs

use glib::translate::ToGlibPtr;
use gtk::glib;
use libc::{c_int, EXIT_SUCCESS};
use std::ffi::CString;

fn main() {
    gtk::init().unwrap();

    let lib_path_str = "target/debug/libgtk_rs_c_api.so";
    let lib_path_cstr = CString::new(lib_path_str).unwrap();

    unsafe {
        let handle = libc::dlopen(lib_path_cstr.as_ptr(), libc::RTLD_NOW);
        if handle.is_null() {
            let error = libc::dlerror();
            let error_message = CString::from_raw(error);
            panic!(
                "Error loading shared library '{}': {}",
                lib_path_str,
                error_message.to_string_lossy()
            );
        }

        let builder = gtk::Builder::from_string(include_str!("main.xml"));
        set_builder(handle, builder);
        let window = gtk::Window::new();
        set_window(handle, window);
        execute(handle);

        libc::dlclose(handle);
    }
}

unsafe fn execute(handle: *mut libc::c_void) {
    let function_name = CString::new("execute").unwrap();
    let function = {
        let ptr = libc::dlsym(handle, function_name.as_ptr());
        let function: Option<extern "C" fn() -> c_int> = std::mem::transmute(ptr);
        function.expect("Error finding symbol {function_name:?} in the shared library.")
    };
    let exit_code = function();
    assert_eq!(exit_code, EXIT_SUCCESS)
}

unsafe fn set_builder(handle: *mut libc::c_void, builder: gtk::Builder) {
    let function_name = CString::new("set_builder").unwrap();
    let function = {
        let ptr = libc::dlsym(handle, function_name.as_ptr());
        let function: Option<extern "C" fn(*mut gtk::ffi::GtkBuilder) -> c_int> =
            std::mem::transmute(ptr);
        function.expect("Error finding symbol {function_name:?} in the shared library.")
    };
    let exit_code = function(builder.to_glib_full());
    assert_eq!(exit_code, EXIT_SUCCESS)
}

unsafe fn set_window(handle: *mut libc::c_void, window: gtk::Window) {
    let function_name = CString::new("set_window").unwrap();
    let function = {
        let ptr = libc::dlsym(handle, function_name.as_ptr());
        let function: Option<extern "C" fn(*mut gtk::ffi::GtkWindow) -> c_int> =
            std::mem::transmute(ptr);
        function.expect("Error finding symbol {function_name:?} in the shared library.")
    };
    let exit_code = function(window.to_glib_full());
    assert_eq!(exit_code, EXIT_SUCCESS)
}
