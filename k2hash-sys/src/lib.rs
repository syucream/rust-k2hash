extern crate libc;

pub type k2h_h = ::libc::uint64_t;
use libc::{c_int, c_char, c_uchar, size_t};

#[link(name = "k2hash")]
extern "C" {
    pub fn k2h_open(filepath: *const c_char, readonly: bool, removefile: bool, 
					fullmap: bool, maskbitcnt: c_int, cmaskbitcnt: c_int,
					maxelementcnt: c_int, pagesize: size_t) -> k2h_h;

	pub fn k2h_close(handle: k2h_h) -> bool;

    pub fn k2h_get_value(handle: k2h_h, pkey: *const c_uchar, keylength: size_t,
                         ppval: *mut *mut c_uchar, pvallength: *mut size_t) -> bool;

    pub fn k2h_get_str_direct_value(handle: k2h_h, pkey: *const c_uchar) -> *const c_uchar;

    pub fn k2h_set_value(handle: k2h_h, pkey: *const c_uchar, keylength: size_t,
						 pval: *const c_uchar, vallength: size_t) -> bool;
}
