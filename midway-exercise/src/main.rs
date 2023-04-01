use clap::Args;
use clap::Parser;
use clap::Subcommand;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result, Write};
use std::path::Path;
#[derive(Subcommand)]
enum Mode {
    Read(ReadArgs),
    Write(WriteArgs),
}
#[derive(Parser)]
#[command(author, version, about)]

struct Opt {
    #[command(subcommand)]
    mode: Mode,
}
#[derive(Args)]
struct WriteArgs {
    word: String,
    translation: String,
}

#[derive(Args)]
struct ReadArgs {
    word: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Word {
    base: String,
    translation: String,
}

fn main() {
    let args = Opt::parse();
    println!("Yo write in a word to get translated");
    let mut map = load_json("./midway-exercise/test");
    match args.mode {
        Mode::Read(args) => match map.get(&args.word) {
            Some(t) => println!("Translation of the word '{}' is '{}'", args.word, t),
            None => println!("Word doesn't exist in the wordlist!"),
        },
        Mode::Write(args) => match map.get(&args.word) {
            Some(_) => println!("Word already in wordlist!"),
            None => {
                map.insert(args.word, args.translation);
                let _ = save_json("./midway-exercise/test", map);
            }
        },
    }
}

//The '?' basically cuts down the match (or if let) to a single line instead of multiple. Both ways
//are illustrated below
fn save_json<P: AsRef<Path>>(path: P, map: HashMap<String, String>) -> Result<()> {
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let buf = serde_json::to_vec(&map)?;
    file.write_all(&buf[..])?;
    Ok(())
}

fn load_json<P: AsRef<Path>>(path: P) -> HashMap<String, String> {
    if let Ok(mut file) = File::open(path) {
        let mut buffer = vec![];
        if file.read_to_end(&mut buffer).is_ok() {
            if let Ok(map) = serde_json::from_slice(&buffer[..]) {
                return map;
            }
        }
    }
    HashMap::new()
}

#[test]
fn test() {
    println!("yoooooooo")
}
