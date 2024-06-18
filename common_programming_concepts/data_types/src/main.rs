use {std::cmp::Ordering, std::io};

fn main() {
    /* TUPLE TYPE */
    /*let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("{five_hundred}");
    let six_point_four = x.1;

    let one = x.2;*/

    /* ARRAY TYPE */
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index: String = String::new();
    let stdin_object: std::io::Stdin = io::stdin();

    stdin_object
        .read_line(&mut index)
        .expect("Failed to read line");

    //convert user input to usize
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    match index.cmp(&a.len()) {
        Ordering::Less => {
            let element = a[index];
            println!("The value of the element at index {index} is {element}");
        }
        Ordering::Equal | Ordering::Greater => {
            println!(
                "Index out of bounds. Index can take values between 0 and {}",
                a.len() - 1
            );
        }
    }
}
