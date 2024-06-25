mod front_of_house;
use front_of_house::hosting;

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // -- snip --
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // Destructure the inner UsState and bind it to `state`
            println!("State Quarter from {:?}!", state); // Use the variable `state` in the println! macro
            25 // Quarters are worth 25 cents, not 1
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x_value) => Some(x_value + 1),
    }
}

fn main() {
    /*let five = Some(5);
    let six = plus_one(five);

    let none = plus_one(None);

    // Testing the value_in_cents function
    let alabama_quarter = Coin::Quarter(UsState::Alabama);
    value_in_cents(alabama_quarter);*/

    hosting::add_to_waitlist();
}
