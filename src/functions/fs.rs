use super::input;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    name: String,
    username: String,
    password: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    string: String,
}
pub fn read_password() -> Vec<Password> {
    let passwords = fs::read_to_string("./passwords.json").expect("Error reading password");
    let password_vec: Vec<Password> = serde_json::from_str(&passwords).unwrap();

    password_vec
}

pub fn save_password() {
    let exists = exists("./passwords.json");
    let name = input("name");
    let username = input("username/email");
    let password = input("Password");

    if !exists {
        let contents = serde_json::to_string(&vec![Password {
            name,
            username,
            password,
        }])
        .expect("Error");
        fs::write("./passwords.json", &contents).expect("Error");
        return println!("{:?}", contents);
    }
    let mut passwords = read_password();
    passwords.push(Password {
        name,
        username,
        password,
    });

    let contents = serde_json::to_string(&passwords).unwrap();
    fs::write("./passwords.json", contents).expect("Error");
}
pub fn passwords() {
    if !exists("./passwords.json") {
        return println!("There are no passwords");
    }

    for data in read_password() {
        let pass = format!(
            "\n   {}\nusername:{}\npassword:{}",
            data.name, data.username, data.password
        );
        println!("{}", pass)
    }
}
pub fn config() -> String {
    if !exists("./config.json") {
        return "1".to_string();
    }
    let cfg = fs::read_to_string("./config.json").expect("Error reading config file");
    let c: Config = serde_json::from_str(&cfg).unwrap();
    c.string
}
pub fn exists(path: &str) -> bool {
    let exists = Path::new(path).exists();
    exists
}
