<img align="right" alt="NVIDIA Logo" width="25%" src="assets/Nvidia-Logo.png">

# nvtx
[![Crates.io](https://img.shields.io/crates/v/nvtx)](https://crates.io/crates/nvtx)
[![Documentation](https://docs.rs/nvtx/badge.svg)](https://docs.rs/nvtx)
[![Build Status](https://github.com/simbleau/nvtx/workflows/build/badge.svg)](https://github.com/simbleau/nvtx/actions/workflows/build.yml)
[![dependency status](https://deps.rs/repo/github/simbleau/nvtx/status.svg)](https://deps.rs/repo/github/simbleau/nvtx)

An safe rust wrapper for the NVIDIA® Tools Extension SDK (NVTX).

NVIDIA® Tools Extension SDK (NVTX) is a C-based Application Programming Interface (API) for annotating events, code ranges, and resources in your applications.
Official documentation for NVIDIA®'s NVTX can be found [here](https://nvidia.github.io/NVTX/doxygen/index.html).

# Motivation

The intent is wrap to safely wrap the NVTX library in "Rust" fashion to provide a proper cross-platform library for GPU and CPU profiling. 

Ideally this library would be used in benchmarking rust applications and performing research on rust projects such as a GPU analysis with zero-cost abstraction.

# Sections

- [nvtx](#nvtx)
- [Motivation](#motivation)
- [Sections](#sections)
  - [Using nvtx](#using-nvtx)
- [Getting Started](#getting-started)
  - [State](#state)
  - [FAQ](#faq)
    - [I need help!](#i-need-help)
  - [Contributing](#contributing)
  - [License](#license)

## Using nvtx

nvtx is designed to be easy to use. The crate is published on [crates.io](https://crates.io/crates/nvtx) and the easiest way to use nvtx is by adding the lone dependency to your `Cargo.toml` file:

```toml
nvtx = "0.11.0"
```

# Getting Started

![Screenshot from 2021-05-24 19-54-07](https://user-images.githubusercontent.com/48108917/119422236-a310bc80-bcce-11eb-960a-ea6e4f681dd8.png)

There are several examples in the [example folder](https://github.com/simbleau/nvtx/tree/main/examples) which can be run from applications such as NVIDIA NSight Systems. Each example has a README with easy to read steps, screenshots, and documentation. Check out the first example, '[Hello, World!](https://github.com/simbleau/nvtx/tree/main/examples/hello_world)'

## State

nvtx is in active development and maturing. Right now development is driven by necessity. In the following months I will be performing a GPU analysis and thus will wrap the appropriate (common) functionality one would need or seem important to me.

## FAQ

### I need help!

Don't hesitate to [file an issue](https://github.com/simbleau/nvtx/issues/new) or contact [@simbleau](https://github.com/simbleau) by [e-mail](mailto:spencer@imbleau.com).

## Contributing

I encourage all contributions by pull request. Please check the [issues](https://github.com/simbleau/nvtx/issues) first if you'd like to help. Another great place to start would be binding more functions and wrapping them in a safe way. I will not accept unsafe functionality unless truly required. FFI handling is tricky, and be aware that there are several inline functions in the NVTX header files. This makes converting them automatically (with tools such as bindgen) more difficult. If you have a good way to do it, I'd love to see it! I ended up going the manual translation route because automatic binding was troublesome.

The [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) applies.

## License

This  project is dual-licensed under both [Apache 2.0](https://github.com/simbleau/nvtx/blob/main/LICENSE-APACHE) and [MIT](https://github.com/simbleau/nvtx/blob/main/LICENSE-MIT) licenses.
