extern crate k2hash_sys;
extern crate libc;

use std::ffi::CString;
use libc::{c_int, size_t};

pub struct K2Hash {
    handler: k2hash_sys::k2h_h,
}

impl K2Hash {
    /// Open a database
    pub fn new(
        filepath: &std::path::Path,
        readonly: bool,
        removefile: bool,
        fullmap: bool,
        maskbitcnt: c_int,
        cmaskbitcnt: c_int,
        maxelementcnt: c_int,
        pagesize: size_t,
    ) -> Result<K2Hash, std::io::Error> {
        let path_str = filepath.to_str().unwrap();
        let c_path = CString::new(path_str).unwrap();
        unsafe {
            let handler = k2hash_sys::k2h_open(
                c_path.as_ptr(),
                readonly,
                removefile,
                fullmap,
                maskbitcnt,
                cmaskbitcnt,
                maxelementcnt,
                pagesize);

            if handler == 0 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(K2Hash { handler: handler })
            }
        }
    }

    /// Set a key/value.
    pub fn set(&self, key: String, value: String) -> Result<(), std::io::Error>
    {
        if self.handler == 0 {
            return Err(std::io::Error::last_os_error())
        }

        println!("{}", self.handler);
        println!("{}", key);
        println!("{}", value);
        unsafe {
            let keylen = key.len();
            let vallen = value.len();
            let ckey = CString::new(key).unwrap();
            let cval = CString::new(value).unwrap();

            let success = k2hash_sys::k2h_set_value(
                self.handler,
                ckey.as_ptr() as *const u8,
                keylen,
                cval.as_ptr() as *const u8,
                vallen);

            if !success {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
    }
}

impl Drop for K2Hash {
    fn drop(&mut self) {
        unsafe {
            k2hash_sys::k2h_close(self.handler);
        }
    }
}
