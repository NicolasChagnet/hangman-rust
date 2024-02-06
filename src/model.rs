use std::fmt;
use crate::io;
use crate::service::MAXGUESSES;

// Game struct containing all relevant info on a given guessing game
pub struct Game {
    word: String, // word to guess
    word_chars: Vec<char>, // list of characters making up the word to guess
    guesses: Vec<char>, // letters already guessed by player
    found: Vec<char>, // correct letters found by player
    masked_word: Vec<Option<char>>, // Mask of letters found by players, with matching position
    attempts: u32 // Record of number of (wrong) guesses
}

// Various functions used to read/update the status of the game
impl Game {
    pub fn new(word_in: String) -> Game {
        let len = word_in.len();
        let chars_l: Vec<char> = word_in.chars().collect();
        Game {
            word: word_in,
            word_chars: chars_l,
            guesses: Vec::new(),
            found: Vec::new(),
            masked_word: vec![None; len],
            attempts: 0
        }
    }

    pub fn is_already_guessed(&self, c: char) -> bool {
        return self.guesses.contains(&c);
    }

    pub fn is_already_found(&self, c: char) -> bool {
        return self.found.contains(&c);
    }

    pub fn is_in_word(&self, c: char) -> bool {
        return self.word_chars.contains(&c);
    }

    pub fn get_attempts(&self) -> u32 {
        return self.attempts;
    }

    pub fn increase_attempts(&mut self) {
        self.attempts += 1;
    }

    pub fn push_guess(&mut self, c: char) {
        self.guesses.push(c);
    }

    pub fn push_found(&mut self, c: char) {
        self.found.push(c);
    }

    pub fn display_guesses(&self) {
        let s: String = match self.guesses.len() {
            0 => "none".to_string(),
            _ => self.guesses.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",") // Join vec of chars
        };
        io::show_message(&format!("Previously tried letters: {}.", &s));
    }
    pub fn display_hangman(&self) {
        let n = self.attempts as usize;
        match self.attempts {
            1..=MAXGUESSES => {
                io::show_message(crate::ascii::ASCII_HANGMAN[n - 1])
            },
            _ => ()
        }
    }
    // Informative piece displaying the status and guesses
    pub fn display_mask(&self) {
        let mut return_string = String::new();
        for c in self.masked_word.iter() {
            match *c {
                Some(ch) => return_string.push(ch),
                None => return_string.push('_')
            };
        }
        io::show_message(&format!("Current status: {}", &return_string));
    }

    // This function both handles the addition of a new valid guess
    // And returns whether the guess is a hit or miss
    pub fn add_guess(&mut self, c: char) -> bool {
        // add character to guesses
        self.guesses.push(c);

        if self.is_in_word(c) {
            // if the character is also in the word, remember it and unmask it
            self.found.push(c);
            for (i, ch) in self.word_chars.iter().enumerate() {
                if c == *ch {
                    self.masked_word[i] = Some(c);
                }
            }
            return true;
        } else {
            return false;
        }
    }

    // Check if the word has been fully found
    pub fn is_finished(&self) -> bool {
        return !(self.masked_word.contains(&None));
    }
}

// Format implementation for our game structure
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_guesses: String = self.guesses.iter().collect();
        write!(f, "The chosen word is {} and the guessed letters are {}.\n {} attempts have been made.", self.word, display_guesses, self.attempts)
    }
}