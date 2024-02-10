use hangman::{get_init_word_from_file, make_guesses, Game, play_again};
use clap::{ArgGroup,Parser};
use random_word::Lang;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[clap(group(
    ArgGroup::new("src")
        .required(false)
        .args(&["word", "file", "random"]),
))]
struct Args {
    /// File contain words to sample
    #[arg(short, long)]
    file: Option<String>,
    /// Choose a specific word
    #[arg(short, long)]
    word: Option<String>,
    /// Pick a random word
    #[arg(short, long)]
    random: bool
}

fn main() {
    // Load CLI arguments
    let args = Args::parse();
    let mut play = true;
    let mut word: String;

    while play {
        // Load word either from file, cli arg or user input
        word = match (&args.file, &args.word, &args.random) {
            (Some(filename), _, _) => get_init_word_from_file(filename),
            (_, Some(w), _) => String::from(w),
            _ => random_word::gen(Lang::En).to_string()
            // _ => unreachable!()
        };
        // match &args.file {
        //     Some(filename) => word = get_init_word_from_file(filename),
        //     None => match &args.word {
        //         Some(w) => word = String::from(w),
        //         None => { word = get_init_word(); clear_screen();}
        //     }
        // }

        // Initialize a game struct
        let game = Game::new(word);
        // Start the guessing on the game
        make_guesses(game);
        play = play_again();
    }
}
