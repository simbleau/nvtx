
# Change Log
All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).
 
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