# nvtx-rs
[![dependency status](https://deps.rs/repo/github/simbleau/nvtx-rs/status.svg)](https://deps.rs/repo/github/simbleau/nvtx-rs)
[![Build Status](https://travis-ci.com/simbleau/nvtx-rs.svg?branch=main)](https://travis-ci.com/simbleau/nvtx-rs) 

An safe rust wrapper for the NVIDIA® Tools Extension SDK (NVTX). \\
More information on NVTX can be found via doxygen [here](https://nvidia.github.io/NVTX/doxygen/index.html).

# Motivation

NVIDIA® Tools Extension SDK (NVTX) is a C-based Application Programming Interface (API) for annotating events, code ranges, and resources in your applications.

The intent is wrap this library safely and in "rust" fashion to have a proper library for GPU and CPU profiling for it to be used in benchmarking applications.

# Sections

* [Motivation](#motivation)
* [State](#state)
* [FAQ](#faq)
* [Contributing](#contributing)
* [License](#license)

## State

nvtx-rs is in active development and still maturing.

## FAQ

### I need help!

Don't hesitate to [file an issue](https://github.com/simbleau/nvtx-rs/issues/new) or contact [@simbleau](https://github.com/simbleau) by [e-mail](mailto:spencer@imbleau.com).

## Contributing

I encourage all contributions and pull requests. A great place to start would be binding more functions and wrapping them in a safe way. FFI is tricky, and be aware that there are several inline functions in the NVTX header files. This makes converting them automatically more difficult. If you have a good way to do it, I'd love to see it! I ended up going the manual route since it seemed too difficult.

The [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) applies.

## License

This  project is dual-licensed under both [Apache 2.0](https://github.com/simbleau/nvtx-rs/blob/main/LICENSE-APACHE) and [MIT](https://github.com/simbleau/nvtx-rs/blob/main/LICENSE-MIT) licenses.
