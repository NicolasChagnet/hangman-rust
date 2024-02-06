use hangman::{clear_screen, get_init_word, get_init_word_from_file, make_guesses, Game};
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

    let args = Args::parse();

    let word: String;

    match args.file {
        Some(filename) => word = get_init_word_from_file(&filename),
        None => match args.word {
            Some(w) => word = w,
            None => { word = get_init_word(); clear_screen();}
        }
    }

    let game = Game::new(word);
    make_guesses(game);
}
