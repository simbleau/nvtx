// Enforce stricter documentation requirements
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::broken_intra_doc_links)]

//! A safe rust FFI binding for the NVIDIA® Tools Extension SDK (NVTX). </br>
//! NVTX API doxygen documentation by NVIDIA® can be found [here](https://nvidia.github.io/NVTX/doxygen/index.html).
use std::ffi::CString;

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
    #[link(name = "nvtx")]
    extern "C" {
        fn _range_push(message: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    }
    let message: CString = str_to_cstring(message);
    unsafe { _range_push(message.as_ptr()) } // SAFETY: this is safe trust me bro
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
        fn _range_pop() -> ::std::os::raw::c_int;
    }
    unsafe { _range_pop() } // SAFETY: this is safe trust me bro
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
        fn _mark(message: *const ::std::os::raw::c_char);
    }
    let message: CString = str_to_cstring(message);
    unsafe { _mark(message.as_ptr()) } // SAFETY: this is safe trust me bro
}

// Helper function to reduce code repetition
fn str_to_cstring(message: &str) -> CString {
    return CString::new(message).expect(&format!("Creation of CString failed from {}", message));
}
