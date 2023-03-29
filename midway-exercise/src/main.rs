use clap::Parser;
use std::collections::HashMap;
use std::io;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, no_binary_name(true))]
struct NipponArgs {
    #[arg(short, long)]
    word: String,
    #[arg(short, long)]
    reverse: Option<String>,
}
fn main() {
    let words = HashMap::from([
        ("gaijin", "foreigner"),
        ("baka", "viktor"),
        ("chinchin", "dick"),
    ]);

    println!("Yo write in a word to get translated");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Shit fucked");
        if input.trim() == "Stop" {
            break;
        }

        let array = input.trim().split(' ');
        let args = NipponArgs::parse_from(array);

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
