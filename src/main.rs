// call_so.rs

use glib::translate::ToGlibPtr;
use gtk::glib;
use libc::c_int;
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
        run(handle);

        libc::dlclose(handle);
    }
}

unsafe fn run(handle: *mut libc::c_void) {
    let function_name = CString::new("run").unwrap();
    let function: Option<extern "C" fn() -> c_int> = {
        let ptr = libc::dlsym(handle, function_name.as_ptr());
        std::mem::transmute(ptr)
    };

    if let Some(run_fn) = function {
        let exit_code = run_fn();
        println!("Exit code: {}", exit_code);
    } else {
        panic!("Error finding symbol {function_name:?} in the shared library.");
    }
}

unsafe fn set_builder(handle: *mut libc::c_void, builder: gtk::Builder) {
    let function_name = CString::new("set_builder").unwrap();
    let function: Option<extern "C" fn(*mut gtk::ffi::GtkBuilder) -> c_int> = {
        let ptr = libc::dlsym(handle, function_name.as_ptr());
        std::mem::transmute(ptr)
    };

    if let Some(run_fn) = function {
        let exit_code = run_fn(builder.to_glib_full());
        println!("Exit code: {}", exit_code);
    } else {
        panic!("Error finding symbol {function_name:?} in the shared library.");
    }
}
