use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};

use super::{add_note, passwords, read_note, save_password};

fn menu() -> usize {
    let options = vec!["passwords", "add password", "add note", "read note"];
    let menu: usize = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();
    menu
}
pub fn check_option() {
    let option = menu();

    match option {
        0 => passwords(),
        1 => save_password(),
        2 => add_note(),
        3 => read_note(),
        _ => println!("Error"),
    }
}
pub fn input(text: &str) -> String {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(text)
        .interact_text()
        .unwrap();
    input
}
