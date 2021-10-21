extern crate libc;
use libc::{c_char, c_int, size_t};

// `libmagic` API as in "magic.h"

// https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
#[repr(C)]
pub struct Magic {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub const MAGIC_NONE: c_int = 0x000000;
pub const MAGIC_DEBUG: c_int = 0x000001;
pub const MAGIC_SYMLINK: c_int = 0x000002;
pub const MAGIC_COMPRESS: c_int = 0x000004;
pub const MAGIC_DEVICES: c_int = 0x000008;
pub const MAGIC_MIME_TYPE: c_int = 0x000010;
pub const MAGIC_CONTINUE: c_int = 0x000020;
pub const MAGIC_CHECK: c_int = 0x000040;
pub const MAGIC_PRESERVE_ATIME: c_int = 0x000080;
pub const MAGIC_RAW: c_int = 0x000100;
pub const MAGIC_ERROR: c_int = 0x000200;
pub const MAGIC_MIME_ENCODING: c_int = 0x000400;
pub const MAGIC_MIME: c_int = (MAGIC_MIME_TYPE | MAGIC_MIME_ENCODING);
pub const MAGIC_APPLE: c_int = 0x000800;
pub const MAGIC_NO_CHECK_COMPRESS: c_int = 0x001000;
pub const MAGIC_NO_CHECK_TAR: c_int = 0x002000;
pub const MAGIC_NO_CHECK_SOFT: c_int = 0x004000;
pub const MAGIC_NO_CHECK_APPTYPE: c_int = 0x008000;
pub const MAGIC_NO_CHECK_ELF: c_int = 0x010000;
pub const MAGIC_NO_CHECK_TEXT: c_int = 0x020000;
pub const MAGIC_NO_CHECK_CDF: c_int = 0x040000;
pub const MAGIC_NO_CHECK_TOKENS: c_int = 0x100000;
pub const MAGIC_NO_CHECK_ENCODING: c_int = 0x200000;

#[deprecated]
pub const MAGIC_NO_CHECK_ASCII: c_int = MAGIC_NO_CHECK_TEXT;

#[deprecated]
pub const MAGIC_NO_CHECK_FORTRAN: c_int = 0x000000;
#[deprecated]
pub const MAGIC_NO_CHECK_TROFF: c_int = 0x000000;

// NOTE: the following are from `file.h`, but part of `magic.h` API
#[cfg(feature = "libmagic-abi-v504")]
pub const FILE_LOAD: c_int = 0;
#[cfg(feature = "libmagic-abi-v504")]
pub const FILE_CHECK: c_int = 1;
#[cfg(feature = "libmagic-abi-v504")]
pub const FILE_COMPILE: c_int = 2;

extern "C" {
    pub fn magic_open(flags: c_int) -> *const Magic;
    pub fn magic_close(cookie: *const Magic);

    #[cfg(feature = "libmagic-abi-v504")]
    pub fn magic_getpath(magicfile: *const c_char, action: c_int) -> *const c_char;
    pub fn magic_file(cookie: *const Magic, filename: *const c_char) -> *const c_char;
    pub fn magic_descriptor(cookie: *const Magic, fd: c_int) -> *const c_char;
    pub fn magic_buffer(cookie: *const Magic, buffer: *const u8, length: size_t) -> *const c_char;

    pub fn magic_error(cookie: *const Magic) -> *const c_char;
    #[must_use]
    pub fn magic_setflags(cookie: *const Magic, flags: c_int) -> c_int;

    #[must_use]
    pub fn magic_load(cookie: *const Magic, filename: *const c_char) -> c_int;
    #[must_use]
    pub fn magic_compile(cookie: *const Magic, filename: *const c_char) -> c_int;
    #[must_use]
    pub fn magic_check(cookie: *const Magic, filename: *const c_char) -> c_int;
    pub fn magic_errno(cookie: *const Magic) -> *const c_int;
}
