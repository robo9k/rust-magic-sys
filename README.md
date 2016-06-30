rust-magic-sys [![Build Status](https://travis-ci.org/robo9k/rust-magic-sys.svg?branch=master)](https://travis-ci.org/robo9k/rust-magic-sys)
==============

[Rust](http://www.rust-lang.org/) declarations for [libmagic](http://darwinsys.com/file/).

---

This [cargo -sys package](http://doc.crates.io/build-script.html#*-sys-packages) provides `libmagic` declarations for e.g. the [`magic` binding](https://github.com/robo9k/rust-magic).


# Usage

[`magic-sys` is available on crates.io](https://crates.io/crates/magic-sys) so you can use it like this (in your `Cargo.toml`):

```toml
[dependencies]
magic-sys = "0.2.0"
```

The `rustdoc` is available on [GitHub Pages](https://robo9k.github.io/rust-magic-sys/magic_sys/).

# Requirements

`libmagic` needs to be installed in a standard location (also see [issue #1](https://github.com/robo9k/rust-magic-sys/issues/1)).

On a Debian based Linux system this can be achieved like this:
```sh
sudo apt-get install libmagic1
```

On RHEL/Cent OS, Gentoo and others you will need to install the `file` package.


On Mac OS X you can use [Homebrew](http://brew.sh/):
```sh
brew install libmagic
```

Feedback for Windows ([issue #2](https://github.com/robo9k/rust-magic-sys/issues/2)) support is appreciated!

# License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
