use super::input;
use super::{decrypt, encrypt, read_config};
use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    name: String,
    username: String,
    password: String,
}
pub fn read_password() -> Vec<Password> {
    let path = read_config().path;
    let passwords = fs::read_to_string(path).expect("Error reading password");
    let password_vec: Vec<Password> = serde_json::from_str(&passwords).unwrap();

    password_vec
}

pub fn save_password() {
    let path = read_config().path;
    let exists = exists(path);
    let name = input("name");
    let username = input("username/email");
    let password = input("Password");

    if !exists {
        let contents = serde_json::to_string(&vec![Password {
            name,
            username: encrypt(username),
            password: encrypt(password),
        }])
        .expect("Error");
        fs::write("./passwords.json", &contents).expect("Error");
        return println!("{:?}", contents);
    }

    let mut passwords = read_password();
    passwords.push(Password {
        name,
        username: encrypt(username),
        password: encrypt(password),
    });

    let contents = serde_json::to_string(&passwords).unwrap();
    fs::write("./passwords.json", contents).expect("Error");
}
pub fn passwords() {
    let path = read_config().path;
    if !exists(path) {
        return println!("There are no passwords");
    }
    let passwords = read_password();
    let mut names: Vec<String> = vec![];
    for data in read_password() {
        names.push(data.name)
    }
    let menu: usize = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .default(0)
        .items(&names)
        .interact()
        .unwrap();
    println!(
        "\nusername:{}\npassword:{}",
        decrypt(&passwords[menu].username),
        decrypt(&passwords[menu].password)
    );
    // for data in read_password() {
    //     let pass = format!(
    //         "\n   {}\nusername:{}\npassword:{}",
    //         data.name,
    //         decrypt(data.username),
    //         decrypt(data.password)
    //     );
    //     println!("{}", pass)
    // }
}
pub fn exists(path: String) -> bool {
    let exists = Path::new(&path).exists();
    exists
}
