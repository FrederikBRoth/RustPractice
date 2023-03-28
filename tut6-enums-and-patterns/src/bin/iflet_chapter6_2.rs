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
    //Match might be wordy if only one expected outcome is needed. This can be done with if let

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Maximum: {}", max),
        _ => (),
    }

    //As the warning above shows, this is very verbose considering the thing we try to achieve
    //
    //Do if let instead

    if let Some(max) = config_max {
        println!("Maximum: {}", max);
    }

    //Can also use an else in the if let to handle the catch-all '_'
    //
    //Match version

    let coin = Coin::Quarter(UsState::Kentucky);
    let coin = Coin::Dime;
    let mut count = 0;
    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }
    println!("{count}");

    //if let
    if let Coin::Quarter(ref state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

    println!("{count}");

    //FYI: ref is used in both statements to ensure borrowing instead of moving since match always
    //moves and consumes by default instead of borrowing (or technically lending lmao)
}
