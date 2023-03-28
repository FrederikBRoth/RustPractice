enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Kentucky,
}
fn main() {
    let coin = Coin::Dime;
    println!("{}", value_in_cents(coin));
    let quarter = Coin::Quarter(UsState::Kentucky);
    println!("{}", value_in_cents(quarter));

    //Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!(
        "Number: {}",
        match six {
            Some(i) => i,
            None => 0,
        }
    );

    println!("Number: {}", six.unwrap_or(0));

    //Catch all in match
    let dice_roll = 9;
    match dice_roll {
        3 => win(),
        7 => lose(),
        _ => try_again(),
    }
}

fn win() {}
fn lose() {}
fn try_again() {}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn plus_one_clippy(x: Option<i32>) -> Option<i32> {
    x.map(|i| i + 1)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
