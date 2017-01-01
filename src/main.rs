#![allow(non_camel_case_types)]

include!("lib.rs");

fn main() {
    let path = std::path::Path::new("/tmp/tmp.k2hash");
    let k2hash = K2Hash::new(path, false, false, true, 8, 4, 32, 128).unwrap();

    k2hash.set("key".to_string(), "value".to_string());

    println!("k2hash test finished!\n");
}
