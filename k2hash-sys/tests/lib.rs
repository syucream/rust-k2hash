#![allow(non_camel_case_types)]

extern crate k2hash_sys;
extern crate libc;

use std::ffi::CString;

fn open_test_handler() -> k2hash_sys::k2h_h {
    let path = "/tmp/tmp.k2hash";
    let c_path = CString::new(path).unwrap();

    unsafe {
        k2hash_sys::k2h_open(c_path.as_ptr(), false, true, true, 8, 4, 32, 128)
    }
}

#[test]
fn test_open() {
    let handler = open_test_handler();

    assert_ne!(handler, 0);
}

#[test]
fn test_set_value() {
    let handler = open_test_handler();

    // {"test_set_value": [1,2,3,4,5]}
    let rkey = vec![0x74, 0x65, 0x73, 0x74, 0x5f, 0x73, 0x65, 0x74, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65];
    let rval = vec![1, 2, 3, 4, 5];

    unsafe {
        let success = k2hash_sys::k2h_set_value(handler, rkey.as_ptr(), rkey.len(), rval.as_ptr(), rval.len());
        assert!(success);
    };
}

#[test]
fn test_set_str_value() {
    let handler = open_test_handler();

    let rkey = "test_set_str_value";
    let rval = "value";
    let ckey = CString::new(rkey).unwrap();
    let cval = CString::new(rval).unwrap();

    unsafe {
        let success = k2hash_sys::k2h_set_str_value(handler, ckey.as_ptr(), cval.as_ptr());
        assert!(success);
    }
}

#[test]
fn test_get_str_direct_value() {
    let handler = open_test_handler();

    // Set a pair for this test
    let rkey = "test_get_str_direct_value";
    let rval = "value";
    let ckey = CString::new(rkey).unwrap();
    let cval = CString::new(rval).unwrap();
    unsafe {
        k2hash_sys::k2h_set_str_value(handler, ckey.as_ptr(), cval.as_ptr());
    }

    unsafe {
        let directval = k2hash_sys::k2h_get_str_direct_value(handler, ckey.as_ptr());
        let rvsize = ::libc::strlen(directval as *const i8);
        let valstr = String::from_raw_parts(directval as *mut _, rvsize, rvsize);

        assert_eq!(valstr, rval);
    }
}
