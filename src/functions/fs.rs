use super::input;
use serde::{Deserialize, Serialize};
use std::fs;
#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    name: String,
    password: String,
}
pub fn read_password() -> Vec<Password> {
    let passwords = fs::read_to_string("./passwords.json").expect("Error reading password");
    let password_vec: Vec<Password> = serde_json::from_str(&passwords).unwrap();

    password_vec
}

pub fn save_password() {
    let name = input("username/email");
    let password = input("Password");
    let mut passwords = read_password();
    passwords.push(Password { name, password });
    let contents = serde_json::to_string(&passwords).unwrap();
    fs::write("./passwords.json", contents).expect("Error");
    println!("{:?}", passwords)
}
