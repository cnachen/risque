#![cfg(target_os = "macos")]

use std::ffi::c_void;

extern "C" {
    pub fn sys_icache_invalidate(start: *mut c_void, len: usize);
}