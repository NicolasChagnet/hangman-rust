use text_io::read;
use std::fs::read_to_string;
use std::io::Result;

// Asks for user input
pub fn get_input(message: &str) -> String {
    println!("{}", message);
    let word: String = read!();
    return word;
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