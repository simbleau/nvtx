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
//! range_push!("Hello World!");
//! // <-- Expensive algorithm here
//! range_pop!();
//! ```
//!
//! ```
//! use nvtx_rs::{mark};
//! mark!("Operation A");
//! // <-- Expensive algorithm here
//! mark!("Operation B");
//! // <-- Expensive algorithm here
//! ```

mod bindings;
#[doc(hidden)] //invisible in the docs
pub mod __private {
    pub use crate::bindings::*;
}

mod macros;
pub use macros::*;
