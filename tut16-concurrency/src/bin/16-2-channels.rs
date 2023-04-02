use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();

    let channel1 = thread::spawn(move || {
        let val = String::from("Yo");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    channel1.join().unwrap();
    //Its important to realize that when sending stuff over channels, you move the variable and not
    //borrow it

    //Multiple sendings

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    let channel2 = thread::spawn(move || {
        let vals = vec![String::from("Hi"), String::from("Peter?")];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let channel3 = thread::spawn(move || {
        let vals = vec![String::from("Hi"), String::from("Peter?")];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    //You can also clone the sender
}
