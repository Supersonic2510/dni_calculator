mod nie;
mod common;

use std::io::stdin;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use once_cell::sync::Lazy;
use common::utils::clear_terminal_screen;
use nie::nie_validator;

static INPUT_VALIDATOR: Lazy<Regex> = Lazy::new(|| Regex::new("^\\b[1|2]\\b$").unwrap());

fn main() {
    // Get DNI (without final letter)
    // Eg. 12345678A
    let mut input = String::new();
    let mut option : u8;

    'menu_loop: loop {

        clear_terminal_screen();

        'input_loop: loop {
            println!("NIE Validator:\n  Select following options:\n    - (1) Validate NIE\n    - (2) Exit\n");

            // Clear the input
            input.clear();

            // Read the new line to get the option
            stdin().read_line(&mut input).expect("Unable to read line");

            //Now get use regex to check if matches options
            match INPUT_VALIDATOR.captures(&input.trim_end()) {
                Some(captures) => {
                    let option_string = &captures[0];
                    option = option_string.parse::<u8>().unwrap();
                    break 'input_loop;
                },
                None => {
                    clear_terminal_screen();
                    println!("Invalid input, please use a valid one\n");
                },
            }
        }

        match option {
            1 => {
                nie_validator::nie_validator();
                sleep(Duration::new(2, 5000));
            },
            2 => break 'menu_loop,
            _ => panic!("Invalid input has accessed option"),
        }
    }

    clear_terminal_screen();
}
