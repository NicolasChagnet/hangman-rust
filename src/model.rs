use std::fmt;
use crate::io;
pub struct Word {
    word: String,
    word_chars: Vec<char>,
    guesses: Vec<char>,
    found: Vec<char>,
    masked_word: Vec<Option<char>>,
    attempts: u32
}

impl Word {
    pub fn new(word_in: String) -> Word {
        let len = word_in.len();
        let chars_l: Vec<char> = word_in.chars().collect();
        Word {
            word: word_in,
            word_chars: chars_l,
            guesses: Vec::new(),
            found: Vec::new(),
            masked_word: vec![None; len],
            attempts: 0
        }
    }

    pub fn is_already_guessed(&self, c: &char) -> bool {
        return self.guesses.contains(c);
    }

    pub fn is_already_found(&self, c: &char) -> bool {
        return self.found.contains(c);
    }

    pub fn is_in_word(&self, c: &char) -> bool {
        return self.word_chars.contains(c);
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
        let mut return_string = String::new();
        for c in self.masked_word.iter() {
            match *c {
                Some(ch) => return_string.push(ch),
                None => return_string.push('_')
            };
        }
        io::display_str(&return_string);
    }

    pub fn add_guess(&mut self, c: char) -> bool {
        self.guesses.push(c);

        if self.is_in_word(&c) {
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

    pub fn is_finished(&self) -> bool {
        return !(self.masked_word.contains(&None));
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_guesses: String = self.guesses.iter().collect();
        write!(f, "The chosen word is {} and the guessed letters are {}.\n {} attempts have been made.", self.word, display_guesses, self.attempts)
    }
}