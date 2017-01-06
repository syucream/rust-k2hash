#[allow(non_camel_case_types)]

extern crate libc;
use libc::{c_int, c_char, c_uchar, size_t, uint64_t};

pub type k2h_h = uint64_t;
pub const DEFAULT_MASK_BITCOUNT: c_int = 8;
pub const DEFAULT_COLLISION_MASK_BITCOUNT: c_int = 4;
pub const DEFAULT_MAX_ELEMENT_CNT: c_int = 32;
pub const MIN_PAGE_SIZE: size_t = 128;

#[link(name = "k2hash")]
extern "C" {
    pub fn k2h_open(filepath: *const c_char, readonly: bool, removefile: bool, 
					fullmap: bool, maskbitcnt: c_int, cmaskbitcnt: c_int,
					maxelementcnt: c_int, pagesize: size_t) -> k2h_h;

	pub fn k2h_close(handle: k2h_h) -> bool;

    pub fn k2h_get_value(handle: k2h_h, pkey: *const c_uchar, keylength: size_t,
                         ppval: *mut *mut u8, pvallength: *mut size_t) -> bool;

    pub fn k2h_get_str_direct_value(handle: k2h_h, pkey: *const c_char) -> *const c_uchar;

    pub fn k2h_set_value(handle: k2h_h, pkey: *const c_uchar, keylength: size_t,
						 pval: *const u8, vallength: size_t) -> bool;

    pub fn k2h_set_str_value(handle: k2h_h, pkey: *const c_char, pval: *const c_char) -> bool;
}
