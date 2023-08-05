use super::{exists, input};
use glob::glob;
use std::fs;
pub fn add_note() {
    let name = input("note name").replace(' ', "_");
    let text = input(format!("note text {}.md", name).as_str());
    if !exists("./notes".to_string()) {
        fs::create_dir("./notes").expect("Error")
    }
    fs::write(format!("./notes/{}.md", name), &text).expect("Error writing file");
    println!("{}{}", name, text);
}
pub fn read_note() {}
