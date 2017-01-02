#![allow(non_camel_case_types)]

include!("lib.rs");

use std::ffi::CString;

fn main() {
    let path = "/tmp/tmp.k2hash";
    let c_path = CString::new(path).unwrap();
    let handler = unsafe { k2h_open(c_path.as_ptr(), false, true, true, 8, 4, 32, 128) };
    println!("handler opened: {}\n", handler);

    let key = "k1";
    let val = "v1";
    let c_key = CString::new(key).unwrap();
    let c_val = CString::new(val).unwrap();
    unsafe { k2h_set_value(handler, c_key.as_ptr() as *const u8, key.len(), c_val.as_ptr() as *const u8, val.len()) };

    let directVal = unsafe { k2h_get_str_direct_value(handler, c_key.as_ptr() as *const u8) };
    let rvsize = unsafe { ::libc::strlen(directVal as *const i8) };
    let valstr = unsafe { String::from_raw_parts(directVal as *mut _, rvsize, rvsize) };
    println!("rv: {}\n", valstr);

    unsafe { k2h_close(handler) };

    println!("k2hash test finished!\n");
}
