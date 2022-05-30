<img align="right" alt="NVIDIA Logo" width="25%" src="assets/Nvidia-Logo.png">

# NVTX
[![Crates.io](https://img.shields.io/crates/v/nvtx)](https://crates.io/crates/nvtx)
[![Documentation](https://docs.rs/nvtx/badge.svg)](https://docs.rs/nvtx)
[![Build Status](https://github.com/simbleau/nvtx/workflows/build/badge.svg)](https://github.com/simbleau/nvtx/actions/workflows/build.yml)
[![dependency status](https://deps.rs/repo/github/simbleau/nvtx/status.svg)](https://deps.rs/repo/github/simbleau/nvtx)

A safe rust wrapper for the NVIDIA® Tools Extension SDK (NVTX) with zero-cost abstraction.

NVIDIA® Tools Extension SDK (NVTX) is a C-based Application Programming Interface (API) for annotating events, code ranges, and resources in your applications.
Official documentation for NVIDIA®'s NVTX can be found [here](https://nvidia.github.io/NVTX/doxygen/index.html).

This library is a wrapper over that SDK, safely, and with zero-cost abstractions. This library facilitates integration into the rich CPU and GPU profiling tools provided by NVIDIA®, such as NSight Systems. The primary motivation for this library is to assist research GPU analysts and bring NVIDIA® tools to Rust.

# ➡️ Quickstart

The crate is published on [crates.io](https://crates.io/crates/nvtx) and the easiest way to use nvtx is by adding the dependency to your `Cargo.toml` file:

```toml
nvtx = "1.1.0"
```

![Hello, World](examples/hello_world/screenshot.png)

There are several examples in the [`examples`](examples) folder which can be executed through tools such as NSight Systems. Each example has a README document with easy to read steps, screenshots, and documentation. Check out the first example, '[Hello, World!](https://github.com/simbleau/nvtx/tree/main/examples/hello_world)'

Don't hesitate to [file an issue](https://github.com/simbleau/nvtx/issues/new) if you need help.

---

## 🤝 Contributing

If you support the project, consider [sponsoring](https://github.com/sponsors/simbleau) or [buying a coffee](https://www.buymeacoffee.com/simbleau). Otherwise, I encourage all contributions by pull request. Please check the [issues](https://github.com/simbleau/nvtx/issues) if you'd like to help. Another great place to start would be binding more functions and wrapping them in a safe way.

The [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) applies.

---

## 📜 Changelog

See the [changelog](CHANGELOG.md).

---

## 🔏 License
This project is dual-licensed under both [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.
