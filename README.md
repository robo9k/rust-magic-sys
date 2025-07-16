rust-magic-sys
[![build status](https://github.com/robo9k/rust-magic-sys/actions/workflows/build.yml/badge.svg)](https://github.com/robo9k/rust-magic-sys/actions/workflows/build.yml)
==============

[Rust](https://www.rust-lang.org/f) declarations for [libmagic](https://www.darwinsys.com/file/).

---

This [cargo -sys package](https://doc.rust-lang.org/cargo/reference/build-scripts.html) provides `libmagic` declarations for e.g. the [`magic` binding](https://crates.io/crates/magic).


# Usage

[`magic-sys` is available on crates.io](https://crates.io/crates/magic-sys) so you can use it like this (in your `Cargo.toml`):

```toml
[dependencies]
magic-sys = "0.3"
```

The `rustdoc` is available on [docs.rs](https://docs.rs/magic-sys).

# Requirements

This crate requires the `libmagic` C library in version 5.39 or newer.

## libmagic

If you don't want to configure the build, `libmagic` has to be available in a standard location
for either `pkg-config` or `vcpkg`, see [Building](#Building).

On a Debian based Linux system such as Ubuntu this can be achieved like this:
```sh
sudo apt-get install libmagic1 libmagic-dev
```

On RHEL/Cent OS, Gentoo and others you will need to install the `file` package.

On Mac OS X you can use [Homebrew](https://brew.sh/):
```sh
brew install libmagic
```

Feedback for Windows ([issue #2](https://github.com/robo9k/rust-magic-sys/issues/2)) support is appreciated!

You can use Microsoft's [`vcpkg`](https://vcpkg.io) via [`cargo-vcpkg`](https://crates.io/crates/cargo-vcpkg):
```sh
cargo install cargo-vcpkg
cargo vcpkg build
```
Afterwards, you can `cargo build` etc. your crate as usual.

## Version features

The `libmagic` API is extended with new backwards-compatible features every now and then.\
To use newly added `libmagic` functionality, you need to use a corresponding `libmagic` version.

You need to specify your `libmagic` version by activating the matching `magic-sys` feature.\
Each API version has a crate feature like "v5-38" (v5.38 is also the default), see [Cargo.toml](Cargo.toml)\
If you use a different version of `libmagic`, adjust your configuration:
```toml
[dependencies.magic-sys]
version = "0.3"
default-features = false
features = ["v5-41"]
```
Note that those version features are additive, so "v5-41" implies "v5-40" and other previous versions.

If you want to use a newer/different `libmagic` version, you will have to [link it](#Building) accordingly.

# MSRV

The Minimum Supported Rust Version (MSRV) is Rust 1.64 or higher.

This version might be changed in the future, but it will be done with a crate version bump.

# Building

To determine which `libmagic` to link against, this crate uses
[`pkg-config`](https://www.freedesktop.org/wiki/Software/pkg-config/)
and [`vcpkg`](https://vcpkg.io/).

The crate does not offer to link a against a bundled `libmagic` version, see [issue #4](https://github.com/robo9k/rust-magic-sys/issues/4).

In general you can link statically or dynamically against `libmagic`.

With static linkage your binary/library includes the `libmagic` code and _does not_ have a run-time dependency.

With dynamic linkage your binary/library _does not_ include the `libmagic` code and _does_ have a run-time dependency on a `libmagic.dll` / `libmagic.so` / `libmagic.dylib` depending on your platform (Windows / Linux / macOS).\
You might have to ship this `libmagic` shared library with your binary/library if you do not expect your users to have a compatible version installed on their system.

You might want to ship a copy of the default `libmagic` / `file` database with your binary/library if you do not expect your users to have a compatible `libmagic` installed on their system.

## pkg-config

The `pkg-config` crate feature uses the [`pkg-config` crate](https://docs.rs/pkg-config), so check its documentation for details.

You can use e.g. the following environment variables:
- `LIBMAGIC_NO_PKG_CONFIG` if set, will skip `pkg-config`
- `LIBMAGIC_STATIC` if set, instructs `pkg-config` to link statically
- `LIBMAGIC_DYNAMIC` if set, instructs `pkg-config` to link dynamically

By default dynamic linkage is used.

## vcpkg

The `vcpkg` crate feature uses the [`vcpkg` crate](https://docs.rs/vcpkg), so check its documentation for details.

You can use e.g. the following environment variables:
- `VCPKGRS_NO_LIBMAGIC` if set, will skip `vcpkg`
- `VCPKGRS_DYNAMIC` if set, instructs `vcpkg` to link dynamically

By default static linkage is used.

You can use `vcpkg` standalone or by using [`cargo-vcpkg`](https://crates.io/crates/cargo-vcpkg).

If you do _not_ use `cargo vcpkg build`, you will have to either
- `vcpkg install libmagic` and set the `VCPKG_ROOT` environment variable for your `vcpkg` root directory
- `vcpkg integrate install` your `vcpkg` root user-wide

## Override

If you disable or skip both `pkg-config` and `vcpkg` the `magic-sys` build script will fail.\
Especially linking statically to `libmagic` requires additional libraries that depend on your version and system.

You can skip the `magic-sys` build script entirely by [overriding it](https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts).\
This is an option if you want to use neither `pkg-config` nor `vcpkg`.

# License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
