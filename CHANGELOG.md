
# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [1.3.0] - 2024-01-18

There are no update steps from the previous version.

### Added

- [Feature](https://github.com/simbleau/nvtx/pull/19)
  Added `range!` macro.

## [1.2.0] - 2023-07-25

There are no update steps from the previous version.

### Added

- Add macros `range_start!` and `range_end!` from the [NVTX API](https://docs.nvidia.com/nsight-visual-studio-edition/nvtx/index.html#nvtx-library-events-range-start-end).

### Changed

- The crate is now using the stable coolchain

### Fixed

- Nothing

## [1.1.1] - 2022-07-10

There are no update steps from the previous version.

### Added

- Nothing

### Changed

- The crate is now `#![no_std]`

### Fixed

- Nothing

## [1.1.1] - 2022-07-10

There are no update steps from the previous version.

### Added

- Nothing

### Changed

- The crate is now `#![no_std]`

### Fixed

- Nothing

---

## [1.1.0] - 2022-05-30

There are no update steps from the previous version.

### Added

- Users can now annotate thread names with `name_thread!(...)` to wrap the canonical `nvtxNameOsThread(uint32_t, const *char)` from NVTX.

### Changed

- Nothing

### Fixed

- Nothing

---

## [1.0.0] - 2022-05-29

To update, replace your function calls with macros, e.g. `range_push("hello")` becomes `range_push!("hello")`

### Added

- Nothing

### Changed

- The `range_push(&str)` function was changed to `range_push!(...)` which uses argument format capturing similar to the `println!(...)` macro. This is a breaking change.
- The `range_pop()` function was changed to `range_push!()`. This is a breaking change.
- The `mark(&str)` function was changed to `mark!(...)` which uses argument format capturing similar to the `println!(...)` macro. This is a breaking change.

### Fixed

- Nothing

---

## [0.2.0] - 2021-05-24

There are no update steps from the previous version.

### Added

- [Feature](https://github.com/simbleau/nvtx/blob/ab656ade3db26c7aea4346ec975730261b6dcd6d/src/lib.rs#L73)
  Users can use the method `mark(&str)` to wrap the canonical `nvtxMarkA(const *char)` from NVTX.
- [Example](https://github.com/simbleau/nvtx/blob/ab656ade3db26c7aea4346ec975730261b6dcd6d/examples/mark)
  Added an example which uses marks.
- [Example](https://github.com/simbleau/nvtx/blob/ab656ade3db26c7aea4346ec975730261b6dcd6d/examples/thread_range)
  Added an example which uses thread ranges.

### Changed

- [Hello, World!](https://github.com/simbleau/nvtx/blob/ab656ade3db26c7aea4346ec975730261b6dcd6d/examples/hello_world)
  Changed the structure of the example to hold its own folder and README.

### Fixed

- Nothing

---

## [0.1.0] - 2021-05-24

There are no update steps to advance to this version as this is the first version.

### Added

- [Feature](https://github.com/simbleau/nvtx/blob/8966d0cf05338c4472657119bf8277fd2f59cc69/src/lib.rs#L38)
  Users can use the method `range_push(&str)` to wrap the canonical `nvtxRangePushA(const *char)` from NVTX.
- [Feature](https://github.com/simbleau/nvtx/blob/8966d0cf05338c4472657119bf8277fd2f59cc69/src/lib.rs#L57)
  Users can use the method `range_pop()` to wrap the canonical `nvtxRangePop(void)` from NVTX.

### Changed

- Nothing

### Fixed

- Nothing
