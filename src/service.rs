use crate::{get_word, io};
use crate::model::Word;
use regex::Regex;

pub fn get_init_word() -> Word {
    
    let mut counter: u32 = 0;
    let mut word: String;
    const MAXTRY: u32 = 10;
    let re = Regex::new(r"^[a-z]+$").unwrap();

    loop {
        counter += 1;

        word = match counter {
            1 => io::get_word("Pick a word: "),
            MAXTRY => {
                io::show_error("Max. attempts reached.");
                std::process::exit(0);
            }
            _ => io::get_word("Pick another word: ")
        };
        
        match re.is_match(&word) {
            false => io::show_error("Enter a valid word (lowercase letters)!"),
            true => return Word::new(word)
        };
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

pub fn make_guesses(mut word: Word) {
    const MAXGUESSES: u32 = 10;
    let mut guess: char;
    let mut counter: u32 = 1;
    let mut success = false;
    while counter <= MAXGUESSES {
        io::show_message(&format!("Attempt number {}", counter));
        word.display_guesses();
        guess = make_guess();
        
        if word.is_already_found(&guess) {
            io::show_error("Already found this letter!");
            continue;
        }

        if word.is_already_guessed(&guess) {
            io::show_error("Already guessed this letter!");
            continue;
        }
        
        let is_found = word.add_guess(guess);
        if !is_found { 
            io::show_message("Too bad, this letter is not in the word!"); 
            counter += 1;
        }

        if word.is_finished() {
            success = true;
            word.display_guesses();
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