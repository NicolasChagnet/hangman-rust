// use text_io::read;
use std::fs::read_to_string;
use std::io::{Result,Write,stdout,stdin};

// Asks for user input
// pub fn get_input(message: &str) -> String {
//     print!("{}", message);
//     let word: String = read!();
//     return word;
// }

fn trim_newline(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}

// Asks for user input
pub fn get_input(message: &str) -> String {
    // Display hint
    print!("{}", message);
    stdout().flush().unwrap();
    // Read input into variable
    let mut word = String::new();
    stdin().read_line(&mut word).unwrap();
    trim_newline(&mut word); // Remove trailing line
    // Trim input before returning
    return word.to_string();
}

// General display function
pub fn show_message(message: &str) {
    println!("{}", message);
}

// Clears the command-line screen to hide the input word
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

// Error display function
pub fn show_error(message: &str) {
    println!("{}", message);
}

// Loads a string from a file, propagates the error
pub fn read_words_from_file(filename: &str) -> Result<String> {
    println!("Loading words from file: {}", filename);

    let text = read_to_string(&filename)?;
    Ok(text)
}