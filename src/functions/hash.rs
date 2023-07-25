use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash_password() {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    let a = s.finish();
    println!("Hash password! {a}")
}
