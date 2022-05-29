// Enforce stricter documentation requirements
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::broken_intra_doc_links)]

//! A safe rust FFI binding for the NVIDIA® Tools Extension SDK (NVTX). </br>
//! NVTX API doxygen documentation by NVIDIA® can be found [here](https://nvidia.github.io/NVTX/doxygen/index.html).
//!
//! # Examples
//!
//! ```
//! use nvtx_rs::{range_pop, range_push};
//! range_push("Hello World!");
//! // <-- Expensive algorithm here
//! range_pop();
//! ```
use std::ffi::CString;

/// Starts a nested thread range.
///
/// # Arguments
///
/// * `message` - The event message associated to this range event.
///
/// # Returns
///
/// * returns the 0 based level of range being started. If an error occurs a
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
    #[link(name = "nvtx")]
    extern "C" {
        fn ffi_range_push(
            message: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    let message: CString = str_to_cstring(message);
    // SAFETY: If the function signature matches nvtx-sys/export.rs
    unsafe { ffi_range_push(message.as_ptr()) }
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
    #[link(name = "nvtx")]
    extern "C" {
        fn ffi_range_pop() -> ::std::os::raw::c_int;
    }
    // SAFETY: If the function signature matches nvtx-sys/export.rs
    unsafe { ffi_range_pop() }
}

/// Marks an instantaneous event in the application.
///
/// # Arguments
///
/// * `message` - The message associated to this marker event.
///
/// # Examples
///
/// ```
/// use nvtx_rs::{mark};
/// mark("Operation A");
/// ```
pub fn mark(message: &str) {
    #[link(name = "nvtx")]
    extern "C" {
        fn ffi_mark(message: *const ::std::os::raw::c_char);
    }
    let message: CString = str_to_cstring(message);
    // SAFETY: If the function signature matches nvtx-sys/export.rs
    unsafe { ffi_mark(message.as_ptr()) }
}

// Helper function to reduce code repetition. Panics if message contains any 0
// bytes.
fn str_to_cstring(message: &str) -> CString {
    return CString::new(message)
        .expect(&format!("Creation of CString failed from {}", message));
}
