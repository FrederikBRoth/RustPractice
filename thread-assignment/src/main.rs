use rand::Rng;
use std::time::Duration;
use std::{sync::Arc, sync::Mutex, thread};
struct Data {
    data: Vec<i32>,
}
fn main() {
    let data = Arc::new(Mutex::new(Data { data: vec![] }));

    let writer_data = data.clone();
    let writer = thread::spawn(move || {
        let mut expr = true;
        while expr {
            let mut locked_data = writer_data.lock().unwrap();
            let random_number = rand::thread_rng().gen_range(1..=5000000);
            locked_data.data.push(random_number);
            println!("Inserted: {}", random_number);
            if locked_data.data.len() > 5 {
                expr = false;
            }
            drop(locked_data);
            thread::sleep(Duration::from_secs(1));
        }
    });

    let reader_data = data.clone();
    let reader = thread::spawn(move || {
        let mut expr = true;
        while expr {
            let locked_data = reader_data.lock().unwrap();
            println!("Length of array is: {}", locked_data.data.len());
            if locked_data.data.len() > 5 {
                expr = false;
            }
            drop(locked_data);
            thread::sleep(Duration::from_millis(500))
        }
    });

    writer.join().unwrap();
    reader.join().unwrap();

    for element in &data.lock().unwrap().data {
        println!("{}", element);
    }
}
