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
}

fn take_ownership(s: String) {
    println!("String: {}", s);
}

fn take_ownership_return(s: String) -> String {
    println!("Thanks, mate: {}", s);
    s
}
