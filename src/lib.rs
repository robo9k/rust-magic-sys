extern crate libc;
use libc::{c_char, c_int, c_void, size_t};

// `libmagic` API as in "magic.h"

// https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
#[repr(C)]
pub struct Magic {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub const MAGIC_NONE: c_int = 0x0000000;
pub const MAGIC_DEBUG: c_int = 0x0000001;
pub const MAGIC_SYMLINK: c_int = 0x0000002;
pub const MAGIC_COMPRESS: c_int = 0x0000004;
pub const MAGIC_DEVICES: c_int = 0x0000008;
pub const MAGIC_MIME_TYPE: c_int = 0x0000010;
pub const MAGIC_CONTINUE: c_int = 0x0000020;
pub const MAGIC_CHECK: c_int = 0x0000040;
pub const MAGIC_PRESERVE_ATIME: c_int = 0x0000080;
pub const MAGIC_RAW: c_int = 0x0000100;
pub const MAGIC_ERROR: c_int = 0x0000200;
pub const MAGIC_MIME_ENCODING: c_int = 0x0000400;
pub const MAGIC_MIME: c_int = (MAGIC_MIME_TYPE | MAGIC_MIME_ENCODING);
pub const MAGIC_APPLE: c_int = 0x000800;
#[cfg(feature = "libmagic-abi-v523")]
pub const MAGIC_EXTENSION: c_int = 0x1000000;
#[cfg(feature = "libmagic-abi-v523")]
pub const MAGIC_COMPRESS_TRANSP: c_int = 0x2000000;
#[cfg(feature = "libmagic-abi-v523")]
pub const MAGIC_NODESC: c_int = (MAGIC_EXTENSION | MAGIC_MIME | MAGIC_APPLE);

pub const MAGIC_NO_CHECK_COMPRESS: c_int = 0x0001000;
pub const MAGIC_NO_CHECK_TAR: c_int = 0x0002000;
pub const MAGIC_NO_CHECK_SOFT: c_int = 0x0004000;
pub const MAGIC_NO_CHECK_APPTYPE: c_int = 0x0008000;
pub const MAGIC_NO_CHECK_ELF: c_int = 0x0010000;
pub const MAGIC_NO_CHECK_TEXT: c_int = 0x0020000;
pub const MAGIC_NO_CHECK_CDF: c_int = 0x0040000;
#[cfg(feature = "libmagic-abi-v538")]
pub const MAGIC_NO_CHECK_CSV: c_int = 0x0080000;
pub const MAGIC_NO_CHECK_TOKENS: c_int = 0x0100000;
pub const MAGIC_NO_CHECK_ENCODING: c_int = 0x0200000;
#[cfg(feature = "libmagic-abi-v535")]
pub const MAGIC_NO_CHECK_JSON: c_int = 0x0400000;

#[cfg(all(feature = "libmagic-abi-v505", not(feature = "libmagic-abi-v510")))]
pub const MAGIC_NO_CHECK_BUILTIN: c_int = 0x3fb000;
#[cfg(all(feature = "libmagic-abi-v510", not(feature = "libmagic-abi-v535")))]
pub const MAGIC_NO_CHECK_BUILTIN: c_int = MAGIC_NO_CHECK_COMPRESS |
MAGIC_NO_CHECK_TAR      |
/* MAGIC_NO_CHECK_SOFT | */
MAGIC_NO_CHECK_APPTYPE  |
MAGIC_NO_CHECK_ELF      |
MAGIC_NO_CHECK_TEXT     |
MAGIC_NO_CHECK_CDF      |
MAGIC_NO_CHECK_TOKENS   |
MAGIC_NO_CHECK_ENCODING;
#[cfg(all(feature = "libmagic-abi-v535", not(feature = "libmagic-abi-v538")))]
pub const MAGIC_NO_CHECK_BUILTIN: c_int = MAGIC_NO_CHECK_COMPRESS |
MAGIC_NO_CHECK_TAR      |
/* MAGIC_NO_CHECK_SOFT | */
MAGIC_NO_CHECK_APPTYPE  |
MAGIC_NO_CHECK_ELF      |
MAGIC_NO_CHECK_TEXT     |
MAGIC_NO_CHECK_CDF      |
MAGIC_NO_CHECK_TOKENS   |
MAGIC_NO_CHECK_ENCODING |
MAGIC_NO_CHECK_JSON |
0;
#[cfg(feature = "libmagic-abi-v538")]
pub const MAGIC_NO_CHECK_BUILTIN: c_int = MAGIC_NO_CHECK_COMPRESS |
MAGIC_NO_CHECK_TAR      |
/* MAGIC_NO_CHECK_SOFT | */
MAGIC_NO_CHECK_APPTYPE  |
MAGIC_NO_CHECK_ELF      |
MAGIC_NO_CHECK_TEXT     |
MAGIC_NO_CHECK_CSV |
MAGIC_NO_CHECK_CDF      |
MAGIC_NO_CHECK_TOKENS   |
MAGIC_NO_CHECK_ENCODING |
MAGIC_NO_CHECK_JSON |
0;

#[deprecated]
pub const MAGIC_NO_CHECK_ASCII: c_int = MAGIC_NO_CHECK_TEXT;

#[deprecated]
pub const MAGIC_NO_CHECK_FORTRAN: c_int = 0x000000;
#[deprecated]
pub const MAGIC_NO_CHECK_TROFF: c_int = 0x000000;

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
