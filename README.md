<img align="right" alt="NVIDIA Logo" width="25%" src="assets/Nvidia-Logo.png">

# nvtx-rs
[![Crates.io](https://img.shields.io/crates/v/nvtx-rs)](https://crates.io/crates/nvtx-rs)
[![Documentation](https://docs.rs/nvtx-rs/badge.svg)](https://docs.rs/nvtx-rs)
[![Build Status](https://github.com/simbleau/nvtx-rs/workflows/build/badge.svg)](https://github.com/simbleau/nvtx-rs/actions/workflows/build.yml)
[![dependency status](https://deps.rs/repo/github/simbleau/nvtx-rs/status.svg)](https://deps.rs/repo/github/simbleau/nvtx-rs)

An safe rust wrapper for the NVIDIA® Tools Extension SDK (NVTX).

NVIDIA® Tools Extension SDK (NVTX) is a C-based Application Programming Interface (API) for annotating events, code ranges, and resources in your applications.
Official documentation for NVIDIA®'s NVTX can be found [here](https://nvidia.github.io/NVTX/doxygen/index.html).

# Motivation

The intent is wrap to safely wrap the NVTX library in "Rust" fashion to provide a proper cross-platform library for GPU and CPU profiling. 

Ideally this library would be used in benchmarking rust applications and performing research on rust projects such as a GPU analysis with zero-cost abstraction.

# Sections

* [Motivation](#motivation)
* [Using nvtx-rs](#using-nvtx-rs)
* [Getting Started](#getting-started)
* [State](#state)
* [FAQ](#faq)
* [Contributing](#contributing)
* [License](#license)

## Using nvtx-rs

nvtx-rs is designed to be easy to use. The crate is published on [crates.io](https://crates.io/crates/nvtx-rs) and the easiest way to use nvtx-rs is by adding the lone dependency to your `Cargo.toml` file:

```toml
nvtx-rs = "0.11.0"
```

# Getting Started

![Screenshot from 2021-05-24 19-54-07](https://user-images.githubusercontent.com/48108917/119422236-a310bc80-bcce-11eb-960a-ea6e4f681dd8.png)

There are several examples in the [example folder](https://github.com/simbleau/nvtx-rs/tree/main/examples) which can be run from applications such as NVIDIA NSight Systems. Each example has a README with easy to read steps, screenshots, and documentation. Check out the first example, '[Hello, World!](https://github.com/simbleau/nvtx-rs/tree/main/examples/hello_world)'

## State

nvtx-rs is in active development and maturing. Right now development is driven by necessity. In the following months I will be performing a GPU analysis and thus will wrap the appropriate (common) functionality one would need or seem important to me.

## FAQ

### I need help!

Don't hesitate to [file an issue](https://github.com/simbleau/nvtx-rs/issues/new) or contact [@simbleau](https://github.com/simbleau) by [e-mail](mailto:spencer@imbleau.com).

## Contributing

I encourage all contributions by pull request. Please check the [issues](https://github.com/simbleau/nvtx-rs/issues) first if you'd like to help. Another great place to start would be binding more functions and wrapping them in a safe way. I will not accept unsafe functionality unless truly required. FFI handling is tricky, and be aware that there are several inline functions in the NVTX header files. This makes converting them automatically (with tools such as bindgen) more difficult. If you have a good way to do it, I'd love to see it! I ended up going the manual translation route because automatic binding was troublesome.

The [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) applies.

## License

This  project is dual-licensed under both [Apache 2.0](https://github.com/simbleau/nvtx-rs/blob/main/LICENSE-APACHE) and [MIT](https://github.com/simbleau/nvtx-rs/blob/main/LICENSE-MIT) licenses.
