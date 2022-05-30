
# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [1.0.0] - 2022-05-29

To update, replace your function calls with macros, e.g. `range_push("hello")` becomes `range_push!("hello")`

### Added

- Nothing

### Changed

- [range_push](https://github.com/simbleau/nvtx-rs/blob/36dfb8fb27550eece1d3ca23d900e0565f1205c3)
  The `range_push(&str)` function was changed to `range_push!(...)` which uses argument format capturing similar to the `println!(...)` macro. This is a breaking change.
- [range_pop](https://github.com/simbleau/nvtx-rs/blob/36dfb8fb27550eece1d3ca23d900e0565f1205c3)
  The `range_pop()` function was changed to `range_push!()`. This is a breaking change.
- [mark](https://github.com/simbleau/nvtx-rs/blob/36dfb8fb27550eece1d3ca23d900e0565f1205c3)
  The `mark(&str)` function was changed to `mark!(...)` which uses argument format capturing similar to the `println!(...)` macro. This is a breaking change.

### Fixed

- Nothing

## [0.11.0] - 2021-05-24

There are no update steps from the previous version.

### Added

- [Feature](https://github.com/simbleau/nvtx-rs/blob/ab656ade3db26c7aea4346ec975730261b6dcd6d/src/lib.rs#L73)
  Users can use the method `mark(&str)` to wrap the canonical `nvtxMarkA(const *char)` from NVTX.
- [Example](https://github.com/simbleau/nvtx-rs/blob/ab656ade3db26c7aea4346ec975730261b6dcd6d/examples/mark)
  Added an example which uses marks.
- [Example](https://github.com/simbleau/nvtx-rs/blob/ab656ade3db26c7aea4346ec975730261b6dcd6d/examples/thread_range)
  Added an example which uses thread ranges.

### Changed

- [Hello, World!](https://github.com/simbleau/nvtx-rs/blob/ab656ade3db26c7aea4346ec975730261b6dcd6d/examples/hello_world)
  Changed the structure of the example to hold its own folder and README.

### Fixed

- Nothing

## [0.1.0] - 2021-05-24

There are no update steps to advance to this version as this is the first version.

### Added

- [Feature](https://github.com/simbleau/nvtx-rs/blob/8966d0cf05338c4472657119bf8277fd2f59cc69/src/lib.rs#L38)
  Users can use the method `range_push(&str)` to wrap the canonical `nvtxRangePushA(const *char)` from NVTX.
- [Feature](https://github.com/simbleau/nvtx-rs/blob/8966d0cf05338c4472657119bf8277fd2f59cc69/src/lib.rs#L57)
  Users can use the method `range_pop()` to wrap the canonical `nvtxRangePop(void)` from NVTX.

### Changed

- Nothing

### Fixed

- Nothing