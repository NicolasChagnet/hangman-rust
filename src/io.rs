use text_io::read;

pub fn get_word(message: &str) -> String {
    println!("{}", message);
    let word: String = read!();
    return word;
}

pub fn show_message(message: &str) {
    println!("{}", message);
}

pub fn display_str(message: &str) {
    println!("Current status: {}", message);
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn show_error(message: &str) {
    println!("{}", message);
}