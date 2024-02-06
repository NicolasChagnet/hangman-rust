use crate::{get_word, io};
use crate::model::Game;
use regex::Regex;
use std::process;
use std::io::ErrorKind;
use rand::seq::SliceRandom;

pub fn get_init_word() -> String {
    
    let mut counter: u32 = 0;
    let mut word: String;
    const MAXTRY: u32 = 3;
    let re_whole_word = Regex::new(r"^[a-z]+$").unwrap();

    loop {
        counter += 1;

        word = match counter {
            1 => io::get_word("Pick a word: "),
            MAXTRY => {
                io::show_error("Max. attempts reached. Restart the program to try again!");
                process::exit(0);
            }
            _ => io::get_word("Pick another word: ")
        };
        
        match re_whole_word.is_match(&word) {
            false => io::show_error("Enter a valid word (lowercase letters)!"),
            true => return word
        };
    }
}

fn is_valid_word(s: &str, re_whole_word: &Regex) -> bool {
    return (!s.is_empty()) && (re_whole_word.is_match(&s))
}

pub fn get_init_word_from_file(filename: &str) -> String {
    let word_str_ret = io::read_words_from_file(filename);
    let re_whole_word = Regex::new(r"^[a-z]+$").unwrap();
    match word_str_ret {
        Err(e) if e.kind() == ErrorKind::NotFound => {
            io::show_error("File not found!");
            process::exit(0)
        },
        Err(e) if e.kind() == ErrorKind::PermissionDenied => {
            io::show_error("Permission denied when attempting to open the file!");
            process::exit(0)
        },
        Err(_) => {
            io::show_error("Some unknown error happened when attempting to open the file!");
            process::exit(0)
        }
        Ok(text) => {
            let text_split: Vec<&str> = text
                                        .split("\n")
                                        .filter(|&s| is_valid_word(s, &re_whole_word))
                                        .collect();
            let mut rng = rand::thread_rng();
            let chosen = text_split.choose(&mut rng);
            match chosen {
                Some(w) => return String::from(*w),
                None => {
                    io::show_error("No words found in file!");
                    process::exit(0)
                }
            }
        }
    }
}

pub fn make_guess() -> char {
    let re_letter = Regex::new(r"^[a-z]$").unwrap();
    let mut letter: String;
    loop {
        letter = get_word("Pick a letter: ");
        match re_letter.is_match(&letter) {
            false => io::show_error("Enter a valid letter (lowercase)"),
            true  => {
                let letter_chars: Vec<char> = letter.chars().collect();
                return letter_chars[0];
            }
        }
    }
}   

pub fn make_guesses(mut game: Game) {
    const MAXGUESSES: u32 = 6;
    let mut guess: char;
    let mut counter: u32 = 1;
    let mut success = false;
    while counter <= MAXGUESSES {
        io::show_message(&format!("Attempt number {}", counter));
        game.display_guesses();
        guess = make_guess();
        
        if game.is_already_found(guess) {
            io::show_error("Already found this letter!");
            continue;
        }

        if game.is_already_guessed(guess) {
            io::show_error("Already guessed this letter!");
            continue;
        }
        
        let is_found = game.add_guess(guess);
        if !is_found { 
            io::show_message("Too bad, this letter is not in the word!"); 
            counter += 1;
        }

        if game.is_finished() {
            success = true;
            game.display_guesses();
            break;
        }
    }
    match success {
        false => {
            io::show_message("Too late, you died!");
        },
        true => {
            io::show_message("Congratulation, you have found the word!");
        }
    }
}