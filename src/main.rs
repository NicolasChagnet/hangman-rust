use hangman::{clear_screen, get_init_word, get_init_word_from_file, make_guesses, Game, play_again};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long)]
    word: Option<String>
}

fn main() {
    // Load CLI arguments
    let args = Args::parse();
    let mut play = true;
    let mut word: String;

    while play {
        // Load word either from file, cli arg or user input
        match &args.file {
            Some(filename) => word = get_init_word_from_file(filename),
            None => match &args.word {
                Some(w) => word = String::from(w),
                None => { word = get_init_word(); clear_screen();}
            }
        }

        // Initialize a game struct
        let game = Game::new(word);
        // Start the guessing on the game
        make_guesses(game);
        play = play_again();
    }
}
