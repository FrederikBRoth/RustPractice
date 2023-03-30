use clap::Parser;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    Read,
    Write,
}
#[derive(Parser, Debug)]
#[command(author, version, about)]

struct Opt {
    #[clap(value_enum, default_value_t=Mode::Read)]
    mode: Mode,
    #[arg(long, default_value = "gaijin")]
    word: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Word {
    base: String,
    translation: String,
}
fn main() {
    let args = Opt::parse();
    let words = HashMap::from([
        ("gaijin", "foreigner"),
        ("baka", "viktor"),
        ("chinchin", "dick"),
    ]);
    println!("Yo write in a word to get translated");
    match args.mode {
        Mode::Read => {
            println!("Reads the current file")
        }
        Mode::Write => {
            let word: &str = &args.word;
            if let Some(w) = words.get(word) {
                println!("Translated word biiitch: {}", w);
            } else {
                println!(
                    "Our very expansive wordlist doesn't contain the word: {}",
                    word
                );
            }
        }
    }
}

#[test]
fn test() {
    println!("yoooooooo")
}