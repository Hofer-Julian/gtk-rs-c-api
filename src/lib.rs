// Add the libc crate
pub mod code;
use libc::c_int;

// ... Your existing code ...

// Modify the main function to have a C-compatible signature
#[no_mangle]
pub extern "C" fn my_main() -> c_int {
    // Call the original main function and return the exit code
    code::main().into()
}
