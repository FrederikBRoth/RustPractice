fn main() {
    ownership();
}

fn ownership() {
    let mut s = String::from("test");
    s.push_str(", yo");
    println!("{s}");

    //The memory allocated to the heap, as is with the String type, is deallocated when the owner,
    //in this case 's', is out of their local scope
    ownership_move();
}

fn ownership_move() {
    // let s1 = String::from("Test");
    // let s2 = s1;
    // println!("{}, world", s1);

    //This is not allowed, due to the move concept. When a heap allocated object is set to another
    //variable it "moves" from the former to the latter, ensuring that when the scope ends, no
    //double .drop (delete in c++) occures

    //If you want to copy data, aka deep copy, you use .clone();

    let s1 = String::from("Test");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);

    //So we have methods that takes these objects now.
    take_ownership(s1);
    //s1 has now moved to the take_ownership function and is now unavailable in this scope. We can
    //fix that with a return variable
    let s1 = take_ownership_return(s2);
    println!("I got it back lets go: {}", s1);

    //This way kinda sucks tho, wish there was a faster and better way. Oh wait there is. Borrowing
    //and references
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("Lenght of string: {len}");
    //You cannot edit a non mutable reference. It needs the mut keyword
    let mut s2 = String::from("Mutable");

    change(&mut s2);

    println!("mutable reference {s2}");

    //Important note, a mutable reference can only be referenced once

    //Slices is a way to reference part of a collection instead of just referencing the entire
    //collection
    //
    //THe initial problem is that we want a function that takes a string of words seperated by
    //spaces and returns the first word it finds. If there is no spaces, returns the entire word.

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let s = String::from("Hello world");

    let word = first_word(&s);

    println!("{s}");
    //Doing it this way is not optimal due to the fact that the string we need to compare index
    //positions might change or become invalid in the future. Therefor we need at way to have the
    //information from the string in the function using some sort of reference. This is where
    //slices come in

    //Rewritten function using slices instead
    fn first_word_slice(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let ss = String::from("Hello Slice!");

    let word = first_word_slice(&ss);

    println!("{word}");

    //Mindblow time. String literals are LITERALY just string slices aka &str. Therefor they can be
    //used interchangably if applied correctly.

    fn first_word_str(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        //This just returns s and not the reference since then it would return a reference of a
        //reference. And we don't want that
        s
    }

    let last_s = String::from("Hello String!");
    let last_literal = "hello Literal!";

    let word = first_word_str(&last_s[..]);
    println!("{word}");
    let word = first_word_str(&last_literal[6..]);
    println!("{word}");
    let word = first_word_str("test");
    println!("{word}");
}

fn change(s: &mut String) {
    s.push_str(" value");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn take_ownership(s: String) {
    println!("String: {}", s);
}

fn take_ownership_return(s: String) -> String {
    println!("Thanks, mate: {}", s);
    s
}
