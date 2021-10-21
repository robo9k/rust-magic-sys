extern crate libc;
#[cfg(feature = "libmagic-abi-v520")]
use libc::c_void;
use libc::{c_char, c_int, size_t};

// `libmagic` API as in "magic.h"

// https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
#[repr(C)]
pub struct Magic {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

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
pub const MAGIC_MIME: c_int = (MAGIC_MIME_TYPE | MAGIC_MIME_ENCODING);
pub const MAGIC_APPLE: c_int = 0x00_0800;
#[cfg(feature = "libmagic-abi-v523")]
pub const MAGIC_EXTENSION: c_int = 0x100_0000;
#[cfg(feature = "libmagic-abi-v523")]
pub const MAGIC_COMPRESS_TRANSP: c_int = 0x200_0000;
#[cfg(feature = "libmagic-abi-v523")]
pub const MAGIC_NODESC: c_int = (MAGIC_EXTENSION | MAGIC_MIME | MAGIC_APPLE);

pub const MAGIC_NO_CHECK_COMPRESS: c_int = 0x000_1000;
pub const MAGIC_NO_CHECK_TAR: c_int = 0x000_2000;
pub const MAGIC_NO_CHECK_SOFT: c_int = 0x000_4000;
pub const MAGIC_NO_CHECK_APPTYPE: c_int = 0x000_8000;
pub const MAGIC_NO_CHECK_ELF: c_int = 0x001_0000;
pub const MAGIC_NO_CHECK_TEXT: c_int = 0x002_0000;
pub const MAGIC_NO_CHECK_CDF: c_int = 0x004_0000;
#[cfg(feature = "libmagic-abi-v538")]
pub const MAGIC_NO_CHECK_CSV: c_int = 0x008_0000;
pub const MAGIC_NO_CHECK_TOKENS: c_int = 0x010_0000;
pub const MAGIC_NO_CHECK_ENCODING: c_int = 0x020_0000;
#[cfg(feature = "libmagic-abi-v535")]
pub const MAGIC_NO_CHECK_JSON: c_int = 0x040_0000;

#[cfg(all(feature = "libmagic-abi-v505", not(feature = "libmagic-abi-v510")))]
pub const MAGIC_NO_CHECK_BUILTIN: c_int = 0x3f_b000;
#[cfg(all(feature = "libmagic-abi-v510", not(feature = "libmagic-abi-v535")))]
pub const MAGIC_NO_CHECK_BUILTIN: c_int = MAGIC_NO_CHECK_COMPRESS |
MAGIC_NO_CHECK_TAR      |
// MAGIC_NO_CHECK_SOFT  |
MAGIC_NO_CHECK_APPTYPE  |
MAGIC_NO_CHECK_ELF      |
MAGIC_NO_CHECK_TEXT     |
MAGIC_NO_CHECK_CDF      |
MAGIC_NO_CHECK_TOKENS   |
MAGIC_NO_CHECK_ENCODING;
#[cfg(all(feature = "libmagic-abi-v535", not(feature = "libmagic-abi-v538")))]
pub const MAGIC_NO_CHECK_BUILTIN: c_int = MAGIC_NO_CHECK_COMPRESS |
MAGIC_NO_CHECK_TAR      |
// MAGIC_NO_CHECK_SOFT  |
MAGIC_NO_CHECK_APPTYPE  |
MAGIC_NO_CHECK_ELF      |
MAGIC_NO_CHECK_TEXT     |
MAGIC_NO_CHECK_CDF      |
MAGIC_NO_CHECK_TOKENS   |
MAGIC_NO_CHECK_ENCODING |
MAGIC_NO_CHECK_JSON;
#[cfg(feature = "libmagic-abi-v538")]
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

#[cfg(feature = "libmagic-abi-v521")]
pub const MAGIC_PARAM_INDIR_MAX: c_int = 0;
#[cfg(feature = "libmagic-abi-v521")]
pub const MAGIC_PARAM_NAME_MAX: c_int = 1;
#[cfg(feature = "libmagic-abi-v521")]
pub const MAGIC_PARAM_ELF_PHNUM_MAX: c_int = 2;
#[cfg(feature = "libmagic-abi-v521")]
pub const MAGIC_PARAM_ELF_SHNUM_MAX: c_int = 3;
#[cfg(feature = "libmagic-abi-v522")]
pub const MAGIC_PARAM_ELF_NOTES_MAX: c_int = 4;
#[cfg(feature = "libmagic-abi-v525")]
pub const MAGIC_PARAM_REGEX_MAX: c_int = 5;
#[cfg(feature = "libmagic-abi-v527")]
pub const MAGIC_PARAM_BYTES_MAX: c_int = 6;
#[cfg(feature = "libmagic-abi-v540")]
pub const MAGIC_PARAM_ENCODING_MAX: c_int = 7;

// NOTE: the following are from `file.h`, but part of `magic.h` API
#[cfg(feature = "libmagic-abi-v504")]
pub const FILE_LOAD: c_int = 0;
#[cfg(feature = "libmagic-abi-v504")]
pub const FILE_CHECK: c_int = 1;
#[cfg(feature = "libmagic-abi-v504")]
pub const FILE_COMPILE: c_int = 2;
#[cfg(feature = "libmagic-abi-v505")]
pub const FILE_LIST: c_int = 3;

extern "C" {
    pub fn magic_open(flags: c_int) -> *const Magic;
    pub fn magic_close(cookie: *const Magic);

    #[cfg(feature = "libmagic-abi-v504")]
    pub fn magic_getpath(magicfile: *const c_char, action: c_int) -> *const c_char;
    pub fn magic_file(cookie: *const Magic, filename: *const c_char) -> *const c_char;
    pub fn magic_descriptor(cookie: *const Magic, fd: c_int) -> *const c_char;
    pub fn magic_buffer(cookie: *const Magic, buffer: *const u8, length: size_t) -> *const c_char;

    pub fn magic_error(cookie: *const Magic) -> *const c_char;
    #[cfg(feature = "libmagic-abi-v532")]
    pub fn magic_getflags(cookie: *const Magic) -> c_int;
    #[must_use]
    pub fn magic_setflags(cookie: *const Magic, flags: c_int) -> c_int;

    #[cfg(feature = "libmagic-abi-v513")]
    pub fn magic_version() -> c_int;
    #[must_use]
    pub fn magic_load(cookie: *const Magic, filename: *const c_char) -> c_int;
    #[cfg(feature = "libmagic-abi-v520")]
    #[must_use]
    pub fn magic_load_buffers(
        cookie: *const Magic,
        buffers: *const *const c_void,
        sizes: *const size_t,
        nbuffers: size_t,
    ) -> c_int;

    #[must_use]
    pub fn magic_compile(cookie: *const Magic, filename: *const c_char) -> c_int;
    #[must_use]
    pub fn magic_check(cookie: *const Magic, filename: *const c_char) -> c_int;
    #[cfg(feature = "libmagic-abi-v505")]
    #[must_use]
    pub fn magic_list(cookie: *const Magic, filename: *const c_char) -> c_int;
    pub fn magic_errno(cookie: *const Magic) -> *const c_int;

    #[cfg(feature = "libmagic-abi-v521")]
    #[must_use]
    pub fn magic_setparam(cookie: *const Magic, param: c_int, value: *const c_void) -> c_int;
    #[cfg(feature = "libmagic-abi-v521")]
    #[must_use]
    pub fn magic_getparam(cookie: *const Magic, param: c_int, value: *mut c_void) -> c_int;
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
            magic_close(std::ptr::null());
        }
    }

    #[cfg(feature = "libmagic-abi-v504")]
    #[test]
    fn test_magic_getpath() {
        unsafe {
            magic_getpath(std::ptr::null(), FILE_CHECK);
        }
    }

    #[test]
    fn test_magic_file() {
        unsafe {
            magic_file(std::ptr::null(), std::ptr::null());
        }
    }

    #[test]
    fn test_magic_descriptor() {
        unsafe {
            magic_descriptor(std::ptr::null(), -1);
        }
    }

    #[test]
    fn test_magic_buffer() {
        unsafe {
            magic_buffer(std::ptr::null(), std::ptr::null(), 0);
        }
    }

    #[test]
    fn test_magic_error() {
        unsafe {
            magic_error(std::ptr::null());
        }
    }

    #[cfg(feature = "libmagic-abi-v532")]
    #[test]
    fn test_magic_getflags() {
        unsafe {
            magic_getflags(std::ptr::null());
        }
    }

    #[test]
    fn test_magic_setflags() {
        unsafe {
            let _ = magic_setflags(std::ptr::null(), MAGIC_NONE);
        }
    }

    #[cfg(feature = "libmagic-abi-v513")]
    #[test]
    fn test_magic_version() {
        unsafe {
            magic_version();
        }
    }

    #[test]
    fn test_magic_load() {
        unsafe {
            let _ = magic_load(std::ptr::null(), std::ptr::null());
        }
    }

    #[cfg(feature = "libmagic-abi-v520")]
    #[test]
    fn test_magic_load_buffers() {
        unsafe {
            let _ = magic_load_buffers(std::ptr::null(), std::ptr::null(), std::ptr::null(), 0);
        }
    }

    #[test]
    fn test_magic_compile() {
        unsafe {
            let _ = magic_compile(std::ptr::null(), std::ptr::null());
        }
    }

    #[test]
    fn test_magic_check() {
        unsafe {
            let _ = magic_check(std::ptr::null(), std::ptr::null());
        }
    }

    #[cfg(feature = "libmagic-abi-v505")]
    #[test]
    fn test_magic_list() {
        unsafe {
            let _ = magic_list(std::ptr::null(), std::ptr::null());
        }
    }

    #[test]
    fn test_magic_errno() {
        unsafe {
            magic_errno(std::ptr::null());
        }
    }

    #[cfg(feature = "libmagic-abi-v521")]
    #[test]
    fn test_magic_setparam() {
        unsafe {
            let _ = magic_setparam(std::ptr::null(), MAGIC_PARAM_INDIR_MAX, std::ptr::null());
        }
    }

    #[cfg(feature = "libmagic-abi-v521")]
    #[test]
    fn test_magic_getparam() {
        unsafe {
            let _ = magic_getparam(
                std::ptr::null(),
                MAGIC_PARAM_INDIR_MAX,
                std::ptr::null_mut(),
            );
        }
    }
}
