use std::ffi::CString;
use std::fmt::Display;

#[doc(hidden)]
pub fn _range_push<M: Display>(message: M) -> i32 {
    #[link(name = "nvtx")]
    extern "C" {
        fn ffi_range_push(
            message: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    let message: CString =
        CString::new(message.to_string()).expect("Invalid thread range name");
    // SAFETY: If the function signature matches nvtx-sys/export.rs
    unsafe { ffi_range_push(message.as_ptr()) }
}

#[doc(hidden)]
pub fn _range_pop() -> i32 {
    #[link(name = "nvtx")]
    extern "C" {
        fn ffi_range_pop() -> ::std::os::raw::c_int;
    }
    // SAFETY: If the function signature matches nvtx-sys/export.rs
    unsafe { ffi_range_pop() }
}

#[doc(hidden)]
pub fn _mark<M: Display>(message: M) {
    #[link(name = "nvtx")]
    extern "C" {
        fn ffi_mark(message: *const ::std::os::raw::c_char);
    }
    let message: CString =
        CString::new(message.to_string()).expect("Invalid marker name");
    // SAFETY: If the function signature matches nvtx-sys/export.rs
    unsafe { ffi_mark(message.as_ptr()) }
}
