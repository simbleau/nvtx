<img align="right" alt="NVIDIA Logo" width="25%" src="assets/Nvidia-Logo.png">

# nvtx-rs
[![Crates.io](https://img.shields.io/crates/v/nvtx-rs)](https://crates.io/crates/nvtx-rs)
[![Documentation](https://docs.rs/nvtx-rs/badge.svg)](https://docs.rs/nvtx-rs)
[![Build Status](https://travis-ci.org/simbleau/nvtx-rs.svg?branch=main)](https://travis-ci.com/simbleau/nvtx-rs)
[![dependency status](https://deps.rs/repo/github/simbleau/nvtx-rs/status.svg)](https://deps.rs/repo/github/simbleau/nvtx-rs)

An safe rust wrapper for the NVIDIA® Tools Extension SDK (NVTX). \
More information on NVTX can be found via doxygen [here](https://nvidia.github.io/NVTX/doxygen/index.html).

# Motivation

NVIDIA® Tools Extension SDK (NVTX) is a C-based Application Programming Interface (API) for annotating events, code ranges, and resources in your applications.

The intent is wrap this library safely and in "Rust" fashion to have a proper library for GPU and CPU profiling. Ideally this library would be used in benchmarking applications.

# Sections

* [Motivation](#motivation)
* [Getting Started](#getting-started)
* [State](#state)
* [FAQ](#faq)
* [Contributing](#contributing)
* [License](#license)

# Getting Started

![Screenshot from 2021-05-24 19-54-07](https://user-images.githubusercontent.com/48108917/119422236-a310bc80-bcce-11eb-960a-ea6e4f681dd8.png)

There are several examples in the [example folder](https://github.com/simbleau/nvtx-rs/tree/main/examples) which can be run from applications such as NVIDIA NSight Systems. Each example has a README with easy to read steps, screenshots, and documentation. Check out the first example, '[Hello, World!](https://github.com/simbleau/nvtx-rs/tree/main/examples/hello_world)'

## State

nvtx-rs is in active development and still maturing.

## FAQ

### I need help!

Don't hesitate to [file an issue](https://github.com/simbleau/nvtx-rs/issues/new) or contact [@simbleau](https://github.com/simbleau) by [e-mail](mailto:spencer@imbleau.com).

## Contributing

I encourage all contributions by pull request. Please check the [issues](https://github.com/simbleau/nvtx-rs/issues) first if you'd like to help. Another great place to start would be binding more functions and wrapping them in a safe way. I will not accept unsafe functionality unless truly required. FFI handling is tricky, and be aware that there are several inline functions in the NVTX header files. This makes converting them automatically (with tools such as bindgen) more difficult. If you have a good way to do it, I'd love to see it! I ended up going the manual translation route because automatic binding was troublesome.

The [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) applies.

## License

This  project is dual-licensed under both [Apache 2.0](https://github.com/simbleau/nvtx-rs/blob/main/LICENSE-APACHE) and [MIT](https://github.com/simbleau/nvtx-rs/blob/main/LICENSE-MIT) licenses.
