extern crate k2hash;
extern crate k2hash_sys;

use k2hash::K2Hash;

fn build_wrapped_k2hash() -> Result<K2Hash, std::io::Error> {
    let path = std::path::Path::new("/tmp/tmp.k2hash");

    K2Hash::new(path,
                false,
                true,
                true,
                k2hash::DEFAULT_MASK_BITCOUNT,
                k2hash::DEFAULT_COLLISION_MASK_BITCOUNT,
                k2hash::DEFAULT_MAX_ELEMENT_CNT,
                k2hash::MIN_PAGE_SIZE
                )
}

#[test]
fn test_new() {
    let result = build_wrapped_k2hash();

    result.unwrap();
    assert!(true);
}

#[test]
fn test_set_str() {
    let k2hash = build_wrapped_k2hash().unwrap();

    let key = "test_set_str";
    let val = "val";
    let result = k2hash.set_str(key.to_string(), val.to_string());

    result.unwrap();
    assert!(true);
}

#[test]
fn test_get_str() {
    let k2hash = build_wrapped_k2hash().unwrap();

    let key = "test_get_str";
    let val = "val";
    k2hash.set_str(key.to_string(), val.to_string()).unwrap();

    let actual = k2hash.get_str(key.to_string()).unwrap();

    assert_eq!(actual, val);
}
