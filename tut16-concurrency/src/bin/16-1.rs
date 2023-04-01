use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello, this is number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hello, this is number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //When using variables from the main thread, you need to move them into the new spawned thread
    //since the newly spawned thread might outlive the main thread or other threads it uses the
    //varaible from

    let array = vec![2, 5, 1, 9];
    let spawned = thread::spawn(move || {
        for v in array {
            println!("This is the array element: {}", v);
        }
    });

    spawned.join().unwrap();
    handle.join().unwrap();
}
