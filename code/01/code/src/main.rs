use std::io;

pub mod generate;
pub mod validate;

enum Menu {
    Generate,
    Validate,
}

fn main() {
    let choice: Menu;
    loop {
        println!("Please choose an option:");
        println!("1. Generate MU system theorems");
        println!("2. Test if a string is part of the MU system");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        choice = match input.trim().parse() {
            Ok(1) => Menu::Generate,
            Ok(2) => Menu::Validate,
            _ => continue,
        };
        break;
    }
    match choice {
        Menu::Generate => generate::generate(),
        Menu::Validate => validate::validate(),
    }
}
