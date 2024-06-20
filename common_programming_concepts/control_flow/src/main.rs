use std::cmp::Ordering;
use std::io;
crate utils::*;

fn main() {
    /* IF CONDITION */
    println!("Type a number of your chocie.");
    let mut number: String = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read the line.");

    let number: u32 = number.trim().parse().expect("A number is expected");

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    //possible because if is an epxression thus it return a value !
    let number = if condition { 5 } else { 6 };

    /* LOOP CONDITION */
    /*let mut cpt: u8 = 0;
    loop {
        if cpt < 255 {
            println!("{}", cpt);
            cpt += 1;
        } else {
            println!("{}", cpt);
            println!("Loop is done.");
            break;
        }
    };*/

    // value from loop can be return
    /*let mut counter = 0;
    let res_loop: u32 = loop {
        counter += 1;

        if counter > 9 {
            break counter;
        }
    };

    println!("value of counter is {counter}");*/

    /* COMPLEXE NESTED LOOP */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut count = 0;
    /* WHILE LOOP */
    while count <= 9 {
        count += 1;
        println!("{count}");
    }

    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("element: {element}");
    }

    for number in (1..=10).rev() {
        println!("{number}");
    }

    
}
