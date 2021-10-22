rust-magic-sys [![linux build status](https://github.com/robo9k/rust-magic-sys/actions/workflows/linux.yml/badge.svg)](https://github.com/robo9k/rust-magic-sys/actions/workflows/linux.yml) [![windows build status](https://github.com/robo9k/rust-magic-sys/actions/workflows/windows.yml/badge.svg)](https://github.com/robo9k/rust-magic-sys/actions/workflows/windows.yml) [![mac build status](https://github.com/robo9k/rust-magic-sys/actions/workflows/mac.yml/badge.svg)](https://github.com/robo9k/rust-magic-sys/actions/workflows/mac.yml)
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

This crate requires the `libmagic` C library in version 5.

You need to specify your `libmagic` version by activating the matching `magic-sys` feature.  
Each API version has a crate feature like "v5-38" (v5.38 is also the default), see [Cargo.toml](Cargo.toml)  
If you use a different version of `libmagic`, adjust your configuration:
```toml
[dependencies.magic-sys]
version = "0.3"
default-features = false
features = ["v5-41"]
```
Note that those version features are additive, so "v5-41" implies "v5-40" and other previous versions.

`libmagic` needs to be installed in a standard location (also see [issue #1](https://github.com/robo9k/rust-magic-sys/issues/1)).

On a Debian based Linux system this can be achieved like this:
```sh
sudo apt-get install libmagic1 libmagic-dev
```

On RHEL/Cent OS, Gentoo and others you will need to install the `file` package.


On Mac OS X you can use [Homebrew](https://brew.sh/):
```sh
brew install libmagic
```

Feedback for Windows ([issue #2](https://github.com/robo9k/rust-magic-sys/issues/2)) support is appreciated!

You can use Microsoft's [`vcpkg`](https://vcpkg.io) via [`vcpkg-rs`](https://docs.rs/vcpkg) and [`cargo-vcpkg`](https://crates.io/crates/cargo-vcpkg).
If you choose the latter, that means you'll have to:
```sh
cargo install cargo-vcpkg
cargo vcpkg build
```
Afterwards, you can `cargo build` etc. your crate as usual.

# MSRV

The Minimum Supported Rust Version (MSRV) is Rust 1.38 or higher.

This version might be changed in the future, but it will be done with a crate version bump.

# Building

By default `libmagic` will be searched in the system library paths. If you need to use a different library or are cross-compiling, you can set the `MAGIC_DIR` and `MAGIC_STATIC` environment variables.

## `MAGIC_DIR`, `<TARGET>_MAGIC_DIR`
Tells `rustc` where to find `libmagic.so` / `libmagic.a`. Can have a target-specific prefix like `X86_64_UNKNOWN_LINUX_MUSL_MAGIC_DIR`

## `MAGIC_STATIC`, `<TARGET>_MAGIC_STATIC`
Controls static linking with `libmagic`. Enabled automatically if there's only a `libmagic.a` in the (provided) search path or if explicitly enabled like `MAGIC_STATIC=true`. Can have a target-specific prefix like `X86_64_UNKNOWN_LINUX_MUSL_MAGIC_STATIC`

Similarly `MAGIC_STATIC=false` can be used to choose to link `libmagic` dynamically.
If unset but both libraries are available, the build will bail out with an error and you have to set one option explicitly.

## vcpkg
The optional `vcpkg` integration has its own set of environment variables, see [`vcpkg` crate docs](https://docs.rs/vcpkg/#environment-variables).
If you do not use `cargo vcpkg build`, you will have to either
* `vcpkg install libmagic` and set the environment variables for your `vcpkg` root directory
* `vcpkg integrate install` your `vcpkg` root user-wide

# License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
