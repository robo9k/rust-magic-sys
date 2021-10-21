extern crate libc;
use libc::{c_char, c_int, size_t};

// https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
#[repr(C)]
pub struct Magic {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    pub fn magic_open(flags: c_int) -> *const Magic;
    pub fn magic_close(cookie: *const Magic);
    pub fn magic_error(cookie: *const Magic) -> *const c_char;
    pub fn magic_errno(cookie: *const Magic) -> *const c_int;
    pub fn magic_descriptor(cookie: *const Magic, fd: c_int) -> *const c_char;
    pub fn magic_file(cookie: *const Magic, filename: *const c_char) -> *const c_char;
    pub fn magic_buffer(cookie: *const Magic, buffer: *const u8, length: size_t) -> *const c_char;
    pub fn magic_setflags(cookie: *const Magic, flags: c_int) -> c_int;
    pub fn magic_check(cookie: *const Magic, filename: *const c_char) -> c_int;
    pub fn magic_compile(cookie: *const Magic, filename: *const c_char) -> c_int;
    pub fn magic_list(cookie: *const Magic, filename: *const c_char) -> c_int;
    pub fn magic_load(cookie: *const Magic, filename: *const c_char) -> c_int;
}
