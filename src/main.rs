#![allow(non_camel_case_types)]

include!("lib.rs");

fn main() {
    let path = std::path::Path::new("/tmp/tmp.k2hash");
    let maskBitCnt = DEFAULT_MASK_BITCOUNT;
    let collisionMaskBitCnt = DEFAULT_COLLISION_MASK_BITCOUNT;
    let maxElementCnt = DEFAULT_MAX_ELEMENT_CNT;
    let pageSize = MIN_PAGE_SIZE;

    let k2hash = K2Hash::new(path, false, false, true,
                             maskBitCnt, collisionMaskBitCnt, maxElementCnt, pageSize).unwrap();

    k2hash.set_str("key".to_string(), "value".to_string());

    println!("{}", k2hash.get_str("key".to_string()).unwrap());

    println!("k2hash test finished!\n");
}
