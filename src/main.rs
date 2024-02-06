use hangman::{get_init_word,clear_screen,make_guesses};

fn main() {
    let game = get_init_word();
    clear_screen();
    make_guesses(game);
}
