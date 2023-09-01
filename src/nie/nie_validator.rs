use crate::common::utils::clear_terminal_screen;
use std::io::stdin;
use once_cell::sync::Lazy;
use regex::Regex;

static NIE_VALIDATOR: Lazy<Regex> = Lazy::new(|| Regex::new("^(?:\\d{8}[A-Z]|[XYZ]\\d{7}[A-Z])$").unwrap());

pub fn nie_validator() {
    let mut nie: String = String::new();

    clear_terminal_screen();

    'input_loop: loop {
        println!("To validate the NIE, enter below the full NIE:\n  [You can use also the foreign NIE]\n");

        // Clear the input
        nie.clear();

        // Read the new line to get the nie
        stdin().read_line(&mut nie).expect("Unable to read line");

        //Now get use regex to check if matches options
        match NIE_VALIDATOR.captures(&nie.trim_end()) {
            Some(captures) => {
                // Get a copy rather than a reference
                let x = &captures[0];
                nie.repl;
                break 'input_loop;
            },
            None => {
                clear_terminal_screen();
                println!("Invalid input, please use a valid one\n");
            },
        }
    }
}