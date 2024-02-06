use crate::{get_input, io};
use crate::model::Game;
use regex::Regex;
use std::process;
use std::io::ErrorKind;
use rand::seq::SliceRandom;
use once_cell::sync::Lazy;

const MAXTRYWORD: u32 = 3; // Maximum amount of wrong attempts a user can get
pub const MAXGUESSES: u32 = crate::ascii::ASCII_HANGMAN.len() as u32; // Maximum guesses before the game ends

// Check validity of word
fn is_valid_word(s: &str) -> bool {
    // The use of the Lazy module allows one to compile the regex only once at first use
    static REWORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z]+$").unwrap());
    return REWORD.is_match(&s)
}

fn is_valid_letter(s: &str) -> bool {
    // The use of the Lazy module allows one to compile the regex only once at first use
    static RELETTER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z]$").unwrap());
    return RELETTER.is_match(&s)
}

// Returns a word from use input
pub fn get_init_word() -> String {
    let mut counter: u32 = 0;
    let mut word: String;
    // Infinite loop, try again if the input is invalid, broken after a maximum amount of attempts or if the input is valid
    loop {
        counter += 1;

        word = match counter {
            1 => io::get_input("Pick a word: "),
            MAXTRYWORD => {
                io::show_error("Max. attempts reached. Restart the program to try again!");
                process::exit(0);
            }
            _ => io::get_input("Pick another word: ")
        };
        // Check the validity of input
        match is_valid_word(&word) {
            false => io::show_error("Enter a valid word!"),
            true => return word
        };
    }
}
// Get a random word from a list of words
pub fn get_init_word_from_file(filename: &str) -> String {
    let word_str_ret = io::read_words_from_file(filename);
    // We try to inform the user about errors as much as possible
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
        // If a proper file was found, we parse it into words
        Ok(text) => {
            let text_split: Vec<&str> = text
                                        .split("\n")
                                        .filter(|&s| is_valid_word(s)) // We filter out invalid words
                                        .collect();
            let mut rng = rand::thread_rng();
            let chosen = text_split.choose(&mut rng); // Choose a random valid word
            match chosen {
                Some(w) => return String::from(*w),
                // If the list of valid words is empty, we handle that error
                None => {
                    io::show_error("No words found in file!");
                    process::exit(0)
                }
            }
        }
    }
}

// We ask the user for input until a valid one is returned
fn make_guess() -> char {
    let mut letter: String;
    loop {
        letter = get_input("Pick a letter: ");
        match is_valid_letter(&letter) {
            false => io::show_error("Please enter a valid letter."),
            true  => {
                let letter_chars: Vec<char> = letter.chars().collect();
                return letter_chars[0];
            }
        }
    }
}   

// This function runs most of the game
pub fn make_guesses(mut game: Game) {
    let mut guess: char;
    // let mut counter: u32 = 1;
    let mut success = false;
    // Loop until the player has won or lost
    // while counter <= MAXGUESSES {
    while game.get_errors() < MAXGUESSES { // Max 7
        // Display status at every loop
        io::show_message(&format!("Errors made: {}", game.get_errors()));
        game.display_mask();
        game.display_guesses();
        guess = make_guess(); // Make the user guess a letter
        
        // Check if that letter was already found/guessed
        if game.is_already_found(guess) {
            io::show_error("Already found this letter!");
            continue;
        }
        if game.is_already_guessed(guess) {
            io::show_error("Already guessed this letter!");
            continue;
        }
        // Check if the letter is a correct guess
        let is_found = game.add_guess(guess);
        if !is_found { 
            io::show_message("Too bad, this letter is not in the word!"); 
            game.increase_attempts(); // Increment if wrong (only if the input is valid)
        }

        game.display_hangman(); // Show the growing hangman
        // Check if the end game condition has been fulfilled
        if game.is_finished() {
            success = true;
            game.display_mask();
            break;
        }
    }
    //Post-game
    post_game(game.get_word(), success);
}

// Handles post-game
fn post_game(word: &str, success: bool) {
    match success {
        false => {
            io::show_message(&format!("Too late, you died! The word was {}", word));
        },
        true => {
            io::show_message("Congratulation, you have found the word!");
        }
    }
}

// Prompts the user to play again
pub fn play_again() -> bool {
    let choice = get_input("Play again? [y/N]: ");
    match choice.as_str() {
        "y" => true,
        _ => false
    }
}

