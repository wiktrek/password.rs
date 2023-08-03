use super::read_config;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn encrypt(text: String) -> String {
    let key = read_config().key;
    let e = new_magic_crypt!(key, 256);
    e.encrypt_str_to_base64(text)
}
pub fn decrypt(text: String) -> String {
    let key = read_config().key;
    let e = new_magic_crypt!(key, 256);
    e.decrypt_base64_to_string(text)
        .expect("Your key doesn't work")
}
