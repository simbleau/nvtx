extern crate alloc;
use alloc::{ffi::CString, string::ToString};
use core::fmt::Display;

#[doc(hidden)]
pub fn _range_push<M: Display>(message: M) -> i32 {
    #[link(name = "nvtx")]
    extern "C" {
        fn ffi_range_push(
            message: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
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
        fn ffi_range_pop() -> ::core::ffi::c_int;
    }
    // SAFETY: If the function signature matches nvtx-sys/export.rs
    unsafe { ffi_range_pop() }
}

#[doc(hidden)]
pub fn _range_start<M: Display>(message: M) -> i32 {
    #[link(name = "nvtx")]
    extern "C" {
        fn ffi_range_start(
            message: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
    let message: CString = CString::new(message.to_string())
        .expect("message contains null terminator");
    // SAFETY: calling foreign function is unsafe
    unsafe { ffi_range_start(message.as_ptr()) }
}

#[doc(hidden)]
pub fn _range_end(id: i32) -> i32 {
    #[link(name = "nvtx")]
    extern "C" {
        fn ffi_range_end(id: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
    // SAFETY: calling foreign function is unsafe
    unsafe { ffi_range_end(id) }
}

#[doc(hidden)]
pub fn _mark<M: Display>(message: M) {
    #[link(name = "nvtx")]
    extern "C" {
        fn ffi_mark(message: *const ::core::ffi::c_char);
    }
    let message: CString =
        CString::new(message.to_string()).expect("Invalid marker name");
    // SAFETY: If the function signature matches nvtx-sys/export.rs
    unsafe { ffi_mark(message.as_ptr()) }
}

#[doc(hidden)]
pub fn _name_thread<M: Display>(name: M) {
    #[link(name = "nvtx")]
    extern "C" {
        fn ffi_name_thread(name: *const ::core::ffi::c_char);
    }
    let name: CString =
        CString::new(name.to_string()).expect("Invalid thread name");
    // SAFETY: If the function signature matches nvtx-sys/export.rs
    unsafe { ffi_name_thread(name.as_ptr()) }
}
