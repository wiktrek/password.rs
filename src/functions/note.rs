use super::{exists, input};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
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
pub fn read_note() {
    let mut arr: Vec<String> = vec![];
    for entry in glob("./notes/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let a = path.to_string_lossy().to_string();
                arr.push(a);
            }
            Err(e) => println!("{:?}", e),
        }
    }
    let menu: usize = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .default(0)
        .items(&arr)
        .interact()
        .unwrap();
    let text = fs::read_to_string(&arr[menu]).expect("error reading note");
    println!("{}", text);
}
