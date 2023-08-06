// call_so.rs

use std::ffi::CString;

// Import the necessary items from the libc crate
use libc::c_int;

fn main() {
    let lib_path_str = "target/debug/libgtk_rs_c_api.so";
    let lib_path_cstr = CString::new(lib_path_str).unwrap();

    unsafe {
        let handle = libc::dlopen(lib_path_cstr.as_ptr(), libc::RTLD_NOW);
        if handle.is_null() {
            let error = libc::dlerror();
            let error_message = CString::from_raw(error);
            eprintln!(
                "Error loading shared library '{}': {}",
                lib_path_str,
                error_message.to_string_lossy()
            );
            return;
        }

        // Call the C API function from the shared library
        #[allow(temporary_cstring_as_ptr)]
        let my_main_fn: Option<extern "C" fn() -> c_int> =
            match libc::dlsym(handle, CString::new("my_main").unwrap().as_ptr()) {
                ptr if !ptr.is_null() => Some(std::mem::transmute(ptr)),
                _ => None,
            };

        if let Some(my_main_fn) = my_main_fn {
            let exit_code = my_main_fn();
            println!("Exit code: {}", exit_code);
        } else {
            eprintln!("Error finding symbol 'my_main' in the shared library.");
        }

        libc::dlclose(handle);
    }
}
