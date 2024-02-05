use hangman::{get_init_word,clear_screen,make_guesses};

fn main() {
    let word_to_guess = get_init_word();
    clear_screen();
    make_guesses(word_to_guess);
}
