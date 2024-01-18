// This crate does not use the standard library
#![no_std]
// Enforce stricter documentation requirements
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::broken_intra_doc_links)]

//! A safe rust FFI binding for the NVIDIA® Tools Extension SDK (NVTX). </br>
//! NVTX API doxygen documentation by NVIDIA® can be found [here](https://nvidia.github.io/NVTX/doxygen/index.html).
//!
//! # Examples
//!
//! ```
//! use nvtx::{range_pop, range_push};
//! range_push!("Hello World!");
//! // <-- Expensive algorithm here
//! range_pop!();
//! ```
//!
//! ```
//! use nvtx::range;
//! let range = range!("Hello World!");
//! // <-- Expensive algorithm here
//! drop(range);
//! ```
//!
//! ```
//! use nvtx::{mark};
//! mark!("Operation A");
//! // <-- Expensive algorithm here
//! mark!("Operation B");
//! // <-- Expensive algorithm here
//! ```
//!
//! ```
//! use std::thread;
//! use nvtx::{name_thread};
//! name_thread!("Thread 1");
//! let handler = thread::spawn(|| {
//!    name_thread!("Thread 2");
//! });
//! handler.join().unwrap();
//! ```
mod bindings;
#[doc(hidden)] //invisible in the docs
pub mod __private {
    pub use crate::bindings::*;
}

mod macros;
pub use macros::*;

/// A guard object representing an open NVTX range. Construct it using the [`range!`] macro. When it
/// goes out of scope it will call [`range_end!`].
pub struct RangeGuard(#[doc(hidden)] pub i32);

impl Drop for RangeGuard {
    fn drop(&mut self) {
        let id = self.0;
        range_end!(id);
    }
}
