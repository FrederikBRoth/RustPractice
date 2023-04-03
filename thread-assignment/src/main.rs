use rand::Rng;
use std::time::Duration;
use std::{sync::Arc, sync::Mutex, thread};
use thiserror::Error;
struct Data {
    data: Vec<i32>,
}

#[derive(Error, Debug)]
enum DataError {
    #[error("Number: {0} Data is not even")]
    NotEven(i32),
    #[error("Number: {0} not divisible by 85")]
    NotCool(i32),
}

fn check_number(number: i32) -> Result<(), DataError> {
    if number % 85 != 0 {
        Err(DataError::NotCool(number))
    } else if number % 2 != 0 {
        Err(DataError::NotEven(number))
    } else {
        Ok(())
    }
}

fn main() {
    //let data = Arc::new(Mutex::new(Data { data: vec![] }));
    let number = 170;
    match check_number(number) {
        Ok(_) => println!("Number fits!"),
        Err(e) => println!("Error: {e}"),
    }

    return;
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
