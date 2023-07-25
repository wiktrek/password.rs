use super::config;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
pub fn hash_password(password: String) {
    let hash_string = config();
    println!("{}", hash_string);
    let mut s = DefaultHasher::new();

    password.hash(&mut s);
    let a = s.finish();
    println!("Hash password! {a}")
}
