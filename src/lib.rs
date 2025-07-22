//! # Features
//!
//! ## Build features
//! - `pkg-config` (_enabled by default_) — Enable build using [`pkg-config`](https://www.freedesktop.org/wiki/Software/pkg-config/) with the [`pkg-config` crate](https://docs.rs/pkg-config)  
//!   Check the [crate README for `pkg-config` configuration details](https://crates.io/crates/magic#pkg-config)
//! - `vcpkg` (_enabled by default_) — Enable build using [`vcpkg`](https://vcpkg.io/) with the [`vcpkg` crate](https://docs.rs/vcpkg)  
//!   Check the [crate README for `vcpkg` configuration details](https://crates.io/crates/magic#vcpkg)
//!
//! ## `libmagic` API features
//! - `v5-40` — Enable [`libmagic` v5.40 API](#libmagic-v540)
//!
//!
//! # `libmagic` changelog
//!
//! The following is a subset of `libmagic` changes that are relevant for this `magic-sys` crate.
//!
//! `magic-sys` implements `libmagic` API v5.38 ..= v5.40.  
//! `magic-sys` requires `libmagic` v5.39 or any newer version to build.  
//!
//! ## `libmagic` v5.38
//!
//! API baseline.  
//!
//! ## `libmagic` v5.39
//!
//! No API changes.  
//! Add `libmagic.pc` to build (statically) with `pkg-config`.  
//!
//! ## `libmagic` v5.40
//!
//! Add [`MAGIC_PARAM_ENCODING_MAX`].  
//!
// not yet implemented
// ## `libmagic` v5.41
//
// No API changes.
//
// ## `libmagic` v5.42
//
// No API changes.
//
// ## `libmagic` v5.43
//
// No API changes.
//
// ## `libmagic` v5.44
//
// Add [`MAGIC_NO_COMPRESS_FORK`].
//
// ## `libmagic` v5.45
//
// Add [`MAGIC_NO_CHECK_SIMH`].
// Add [`MAGIC_PARAM_ELF_SHSIZE_MAX`].
//
// ## `libmagic` v5.46
//
// No API changes.
//

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]
// Technically this crate doesn't need Rust `std`
// but you'll still have to get the `libmagic` C library working for your target
#![cfg_attr(not(test), no_std)]

use core::ffi::c_void;
use core::ffi::{c_char, c_int};
use usize as size_t; // core::ffi::c_size_t is unstable, but "is currently always usize"

// `libmagic` API as in "magic.h"

#[allow(non_camel_case_types)]
// https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
#[repr(C)]
pub struct magic_set {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[allow(non_camel_case_types)]
pub type magic_t = *mut magic_set;

pub const MAGIC_NONE: c_int = 0x000_0000;
pub const MAGIC_DEBUG: c_int = 0x000_0001;
pub const MAGIC_SYMLINK: c_int = 0x000_0002;
pub const MAGIC_COMPRESS: c_int = 0x000_0004;
pub const MAGIC_DEVICES: c_int = 0x000_0008;
pub const MAGIC_MIME_TYPE: c_int = 0x000_0010;
pub const MAGIC_CONTINUE: c_int = 0x000_0020;
pub const MAGIC_CHECK: c_int = 0x000_0040;
pub const MAGIC_PRESERVE_ATIME: c_int = 0x000_0080;
pub const MAGIC_RAW: c_int = 0x000_0100;
pub const MAGIC_ERROR: c_int = 0x000_0200;
pub const MAGIC_MIME_ENCODING: c_int = 0x000_0400;
pub const MAGIC_MIME: c_int = MAGIC_MIME_TYPE | MAGIC_MIME_ENCODING;
pub const MAGIC_APPLE: c_int = 0x00_0800;
pub const MAGIC_EXTENSION: c_int = 0x100_0000;
pub const MAGIC_COMPRESS_TRANSP: c_int = 0x200_0000;
pub const MAGIC_NODESC: c_int = MAGIC_EXTENSION | MAGIC_MIME | MAGIC_APPLE;

pub const MAGIC_NO_CHECK_COMPRESS: c_int = 0x000_1000;
pub const MAGIC_NO_CHECK_TAR: c_int = 0x000_2000;
pub const MAGIC_NO_CHECK_SOFT: c_int = 0x000_4000;
pub const MAGIC_NO_CHECK_APPTYPE: c_int = 0x000_8000;
pub const MAGIC_NO_CHECK_ELF: c_int = 0x001_0000;
pub const MAGIC_NO_CHECK_TEXT: c_int = 0x002_0000;
pub const MAGIC_NO_CHECK_CDF: c_int = 0x004_0000;
pub const MAGIC_NO_CHECK_CSV: c_int = 0x008_0000;
pub const MAGIC_NO_CHECK_TOKENS: c_int = 0x010_0000;
pub const MAGIC_NO_CHECK_ENCODING: c_int = 0x020_0000;
pub const MAGIC_NO_CHECK_JSON: c_int = 0x040_0000;

pub const MAGIC_NO_CHECK_BUILTIN: c_int = MAGIC_NO_CHECK_COMPRESS |
MAGIC_NO_CHECK_TAR      |
// MAGIC_NO_CHECK_SOFT  |
MAGIC_NO_CHECK_APPTYPE  |
MAGIC_NO_CHECK_ELF      |
MAGIC_NO_CHECK_TEXT     |
MAGIC_NO_CHECK_CSV      |
MAGIC_NO_CHECK_CDF      |
MAGIC_NO_CHECK_TOKENS   |
MAGIC_NO_CHECK_ENCODING |
MAGIC_NO_CHECK_JSON;

#[deprecated]
pub const MAGIC_NO_CHECK_ASCII: c_int = MAGIC_NO_CHECK_TEXT;

#[deprecated]
pub const MAGIC_NO_CHECK_FORTRAN: c_int = 0x00_0000;
#[deprecated]
pub const MAGIC_NO_CHECK_TROFF: c_int = 0x00_0000;

// TODO: MAGIC_VERSION string

// TODO: MAGIC_SNPRINTB bytes

pub const MAGIC_PARAM_INDIR_MAX: c_int = 0;
pub const MAGIC_PARAM_NAME_MAX: c_int = 1;
pub const MAGIC_PARAM_ELF_PHNUM_MAX: c_int = 2;
pub const MAGIC_PARAM_ELF_SHNUM_MAX: c_int = 3;
pub const MAGIC_PARAM_ELF_NOTES_MAX: c_int = 4;
pub const MAGIC_PARAM_REGEX_MAX: c_int = 5;
pub const MAGIC_PARAM_BYTES_MAX: c_int = 6;
#[cfg_attr(docsrs, doc(cfg(feature = "v5-40")))]
#[cfg(feature = "v5-40")]
pub const MAGIC_PARAM_ENCODING_MAX: c_int = 7;

// NOTE: the following are from `file.h`, but part of `magic.h` API
pub const FILE_LOAD: c_int = 0;
pub const FILE_CHECK: c_int = 1;
pub const FILE_COMPILE: c_int = 2;
pub const FILE_LIST: c_int = 3;

extern "C" {
    pub fn magic_open(flags: c_int) -> magic_t;
    pub fn magic_close(cookie: magic_t);

    pub fn magic_getpath(magicfile: *const c_char, action: c_int) -> *const c_char;
    pub fn magic_file(cookie: magic_t, filename: *const c_char) -> *const c_char;
    pub fn magic_descriptor(cookie: magic_t, fd: c_int) -> *const c_char;
    pub fn magic_buffer(cookie: magic_t, buffer: *const c_void, length: size_t) -> *const c_char;

    pub fn magic_error(cookie: magic_t) -> *const c_char;
    pub fn magic_getflags(cookie: magic_t) -> c_int;
    #[must_use]
    pub fn magic_setflags(cookie: magic_t, flags: c_int) -> c_int;

    pub fn magic_version() -> c_int;
    #[must_use]
    pub fn magic_load(cookie: magic_t, filename: *const c_char) -> c_int;
    #[must_use]
    pub fn magic_load_buffers(
        cookie: magic_t,
        buffers: *mut *mut c_void,
        sizes: *mut size_t,
        nbuffers: size_t,
    ) -> c_int;

    #[must_use]
    pub fn magic_compile(cookie: magic_t, filename: *const c_char) -> c_int;
    #[must_use]
    pub fn magic_check(cookie: magic_t, filename: *const c_char) -> c_int;
    #[must_use]
    pub fn magic_list(cookie: magic_t, filename: *const c_char) -> c_int;
    pub fn magic_errno(cookie: magic_t) -> c_int;

    #[must_use]
    pub fn magic_setparam(cookie: magic_t, param: c_int, value: *const c_void) -> c_int;
    #[must_use]
    pub fn magic_getparam(cookie: magic_t, param: c_int, value: *mut c_void) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Those tests are mostly just sanity checks to see if linkage works,
    // they are NOT testing `libmagic` API/implementation

    #[test]
    fn test_magic_open() {
        unsafe {
            magic_open(MAGIC_NONE);
        }
    }

    #[test]
    fn test_magic_close() {
        unsafe {
            magic_close(std::ptr::null_mut());
        }
    }

    #[test]
    fn test_magic_getpath() {
        unsafe {
            magic_getpath(std::ptr::null(), FILE_CHECK);
        }
    }

    #[test]
    fn test_magic_file() {
        unsafe {
            magic_file(std::ptr::null_mut(), std::ptr::null());
        }
    }

    #[test]
    fn test_magic_descriptor() {
        unsafe {
            magic_descriptor(std::ptr::null_mut(), -1);
        }
    }

    #[test]
    fn test_magic_buffer() {
        unsafe {
            magic_buffer(std::ptr::null_mut(), std::ptr::null(), 0);
        }
    }

    #[test]
    fn test_magic_error() {
        unsafe {
            magic_error(std::ptr::null_mut());
        }
    }

    #[test]
    fn test_magic_getflags() {
        unsafe {
            magic_getflags(std::ptr::null_mut());
        }
    }

    #[test]
    fn test_magic_setflags() {
        unsafe {
            let _ = magic_setflags(std::ptr::null_mut(), MAGIC_NONE);
        }
    }

    #[test]
    fn test_magic_version() {
        unsafe {
            magic_version();
        }
    }

    #[test]
    fn test_magic_load() {
        unsafe {
            let _ = magic_load(std::ptr::null_mut(), std::ptr::null());
        }
    }

    #[test]
    fn test_magic_load_buffers() {
        unsafe {
            let _ = magic_load_buffers(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
            );
        }
    }

    #[test]
    fn test_magic_compile() {
        unsafe {
            let _ = magic_compile(std::ptr::null_mut(), std::ptr::null());
        }
    }

    #[test]
    fn test_magic_check() {
        unsafe {
            let _ = magic_check(std::ptr::null_mut(), std::ptr::null());
        }
    }

    #[test]
    fn test_magic_list() {
        unsafe {
            let _ = magic_list(std::ptr::null_mut(), std::ptr::null());
        }
    }

    #[test]
    fn test_magic_errno() {
        unsafe {
            magic_errno(std::ptr::null_mut());
        }
    }

    #[test]
    fn test_magic_setparam() {
        unsafe {
            let _ = magic_setparam(
                std::ptr::null_mut(),
                MAGIC_PARAM_INDIR_MAX,
                std::ptr::null(),
            );
        }
    }

    #[test]
    fn test_magic_getparam() {
        unsafe {
            let _ = magic_getparam(
                std::ptr::null_mut(),
                MAGIC_PARAM_INDIR_MAX,
                std::ptr::null_mut(),
            );
        }
    }
}
