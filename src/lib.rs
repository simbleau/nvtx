// Enforce stricter documentation requirements
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::broken_intra_doc_links)]

//! A safe rust FFI binding for the NVIDIA® Tools Extension SDK (NVTX).
//! NVTX API documentation by NVIDIA® can be found via [doxygen](https://nvidia.github.io/NVTX/doxygen/index.html).
use std::ffi::CString;

// Import the foreign function interface from a C calling convention in nvtx.c
// Functions exported must match their signature exactly.
extern "C" {
    #![allow(missing_docs)] // These are external functions with their own documentation
    fn rangePush(message: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    fn rangePop() -> ::std::os::raw::c_int;
}

/// Starts a nested thread range.
///
/// # Arguments
///
/// * `message` - The event message associated to this range event.
///
/// # Returns
///
/// * returns the 0 based level of range being started.  If an error occurs a
/// negative value is returned.
///
/// # Examples
///
/// ```
/// use nvtx_rs::{range_pop, range_push};
/// range_push("Hello World!");
/// range_pop();
/// ```
pub fn range_push(message: &str) -> i32 {
    let c_message = CString::new(message).expect("CString failure");
    unsafe { rangePush(c_message.as_ptr()) } // SAFETY: this is safe trust me bro
}

/// Ends a nested thread range.
///
/// # Returns
///
/// * returns the level of the range being ended. If an error occurs a negative
/// value is returned on the current thread.
///
/// # Examples
///
/// ```
/// use nvtx_rs::{range_pop, range_push};
/// range_push("Hello World!");
/// range_pop();
/// ```
pub fn range_pop() -> i32 {
    unsafe { rangePop() } // SAFETY: this is safe trust me bro
}
