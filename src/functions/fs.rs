use std::fs;
pub fn read_password() -> String {
    fs::read_to_string("./passwords.json").expect("Error reading password")
}

pub fn save_password() {
    fs::write("./passwords.json", format!("{}", read_password())).expect("Error");
}
