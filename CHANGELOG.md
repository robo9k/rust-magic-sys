# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0-alpha.1](https://github.com/robo9k/rust-magic-sys/compare/v0.3.0...v0.4.0-alpha.1) - 2025-07-28

This version adds `pkg-config` as an alternative default feature to the existing `vcpkg` build. The `vcpkg` build is now an optional default feature.  
The optional dependency on `pkg-config` bumps the build requirement on the `libmagic` C library to version 5.39 (from 2020-06-14) or newer.  
Naive linking as a fallback is _no longer supported_; if you want to use neither `pkg-config` nor `vcpkg` you have to override the build script, see crate `README`.  

The following environment variables are _no longer supported_ to configure the build, check the crate `README` for alternatives: `MAGIC_DIR` and `<TARGET>_MAGIC_DIR`, `MAGIC_STATIC` and `<TARGET>_MAGIC_STATIC`

The following crate features are _no longer supported_ to select the `libmagic` API version: `v5-04`, `v5-05`, `v5-10`, `v5-13`, `v5-20`, `v5-21`, `v5-22`, `v5-23`, `v5-25`, `v5-27`, `v5-32`, `v5-35`, `v5-38`  
The lowest `libmagic` API supported by this crate is now v5.38.
If you have previously relied on the crate's default features, the `libmagic` API version of v5.38 remains unchanged.  

The minimum supported Rust version (MSRV) is now 1.64

### Added
- [**breaking**] Use `pkg-config` to link, requiring `libmagic` v5.39+
- Set `include` metadata from `pkg-config` and `vcpkg` build

### Fixed

- [**breaking**] Fix C type of `buffer` param in `magic_buffer`

### Other

- Bump MSRV to 1.64
- Replace `libc` crate dependency with `core::ffi`
- Update pinned revision for `cargo-vcpkg`
