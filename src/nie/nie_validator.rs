use crate::{
    common::utils::clear_terminal_screen,
    nie::NIE
};
use std::io::stdin;
use std::num::ParseIntError;
use once_cell::sync::Lazy;
use regex::{Regex};
use unicode_segmentation::UnicodeSegmentation;

static NIE_VALIDATOR: Lazy<Regex> = Lazy::new(|| Regex::new("^(\\d{8}[A-Z])|([XYZ]\\d{7}[A-Z])$").unwrap());

// Take into account the grapheme not UTF-8 char as some languages do not correspond a char to a letter
const LETTER_TABLE: [&str; 23] = ["T", "R", "W", "A", "G", "M", "Y", "F", "P", "D", "X", "B",
    "N", "J", "Z", "S", "Q", "V", "H", "L", "C", "K", "E"];

const FOREIGN_LETTER_TABLE: [&str; 3] = ["X", "Y", "Z"];

pub fn nie_validator() {
    let mut nie_serial: String = String::new();
    let nie: NIE;

    clear_terminal_screen();

    'input_loop: loop {
        println!("To validate the NIE, enter below the full NIE:\n  [You can also use the foreign NIE]\n");

        // Clear the input
        nie_serial.clear();

        // Read the new line to get the nie
        stdin().read_line(&mut nie_serial).expect("Unable to read line");

        //Now get use regex to check if matches options
        match NIE_VALIDATOR.captures(&nie_serial.trim_end()) {
            Some(captures) => {
                // Get a copy rather than a reference
                let serial = String::from(captures.get(0).unwrap().as_str());

                // Check which NIE is
                if captures.get(1) != None {
                    nie = NIE::Local { serial}
                }else if captures.get(2) != None {
                    nie = NIE::Foreign { serial}
                }else {
                    panic!("Invalid input has accessed option")
                }

                break 'input_loop;
            },
            None => {
                clear_terminal_screen();
                println!("Invalid input, please use a valid one\n");
            },
        }
    }

    match validate_nie(&nie) {
        true => println!("\nThis nie: {{{nie}}} is valid!"),
        false => println!("\nThis nie: {{{nie}}} is invalid!"),
    }
}

fn validate_nie(nie: &NIE) -> bool {
    return match nie {
        NIE::Local { serial } => {
            // Split the string into graphemes
            let graphemes: Vec<&str> = UnicodeSegmentation::graphemes(serial.as_str(), true).collect();

            check_letter(&graphemes)
        }
        NIE::Foreign { serial } => {
            // Split the string into graphemes
            let mut graphemes: Vec<&str> = UnicodeSegmentation::graphemes(serial.as_str(), true).collect();

            match FOREIGN_LETTER_TABLE.iter().position(|&letter| letter == graphemes[0]) {
                Some(position_array) => {
                    let position_string: String = position_array.to_string();

                    graphemes[0] = position_string.as_str();

                    check_letter(&graphemes)
                }
                None => false,
            }
        }
    }
}

fn check_letter(graphemes: &Vec<&str>) -> bool {
    let number: Result<u32, ParseIntError> = graphemes[..=graphemes.len() - 2].join("").parse::<u32>();

    match number {
        Ok(number) => {
            return LETTER_TABLE[(number % 23) as usize] == graphemes[graphemes.len() - 1];
        }
        Err(_) => panic!("Invalid NIE has accessed validation"),
    }
}