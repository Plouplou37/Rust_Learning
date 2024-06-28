use std::cmp::Ordering;
use std::collections::btree_map::Range;
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    //Use option for user_preference because it can be (value or None)
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) // unwrap_or_else use LAZINESS in evaluating the default value. This means the closure provided to unwrap is only calle if the Option is None.

        /*match user_preference {
            Some(preference) => preference,
            None => self.most_stocked(),
        }*/
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        match &num_blue.cmp(&num_red) {
            Ordering::Greater => ShirtColor::Blue,
            Ordering::Less => ShirtColor::Red,
            Ordering::Equal => ShirtColor::Blue,
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Red); //Because user_preference is an Option
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let mut list = [
        Rectangle { width: 10 },
        Rectangle { width: 3 },
        Rectangle { width: 7 },
        Rectangle { width: 100 },
        Rectangle { width: 100 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        sort_operations.push(&value);
        r.width
    });

    println!("{:#?}", list[0].width);
    println!("{:#?}", sort_operations.len());

    /*let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}")) // the move transfer the ownership of data from parent thread to child thread.
        .join()
        .unwrap();

    //println!("After child thread: {list:?}");// cannot here because ownership of list is now belonging to child thread.*/

    /*let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut mut_borrows = || list.push(7);
    //println!("Before calling closure: {list:?}");// cannot use println here because the mut_borrow is accessing the mut list in mutable reference. So, no other reference can acces the list untill the mut_borrow is consume !
    mut_borrows();//mut_borrow is consume here so we can use a reference and ptinln! after on list !
    println!("After calling closure: {list:?}");*/
}
