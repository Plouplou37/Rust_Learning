#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    //constructor using contract to validate attributes.
    pub fn new(value: i32) -> Guess {
        //Contract to validate the data
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value } // return a Guess and initialize the value attribute
    }

    //getter because value attrivute is PRIVATE ! We can only get but not modify the value of a GUESS object !
    pub fn value(&self) -> i32 {
        self.value
    }
}
fn main() {
    let guess = Guess::new(-10);
    dbg!(guess);
}
