# rust-k2hash

Rust binding of [yahoojapan/k2hash](https://github.com/yahoojapan/k2hash)

# Examples

* Open and store a new key/value:

```rust
extern crate k2hash;
use k2hash;

fn main() {
    let path = std::path::Path::new("/tmp/tmp.k2hash");
    let k2hash = K2Hash::new(path, false, false, true, 8, 4, 32, 128).unwrap();

    k2hash.set_str("key".to_string(), "value".to_string());
    let value = k2hash.get_str("key".to_string()).unwrap();
}
```

# License

MITL
