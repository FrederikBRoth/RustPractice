use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number boyoooo");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the goddamn line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number dumbass");
                continue;
            }
        };

        println!("You guessed {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big uwu"),
            Ordering::Equal => {
                println!("You win babyyyy");
                break;
            }
        }
    }
}
