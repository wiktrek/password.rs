use std::fs;

#[tokio::main]
async fn main() {
    println!("{}", read_password());

    println!("Hello, world!")
}
fn read_password() -> String {
    fs::read_to_string("./passwords.json").expect("Error reading password")
}
