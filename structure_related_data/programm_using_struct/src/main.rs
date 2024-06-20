// calculate the AREA of RECTANGLE, using its WIDTH and his HEIGHT fields !!
// This sentence give proper clue of how to implement the solution using struct.
#[derive(Debug)] // attribute derive, enable to derive Debug trait capacities
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
fn main() {
    let width1 = 30;
    let height1 = 50;

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    // Here the function only borrow the instance rect1 because we want maybe to use rect1 after in the main. If not it will change owenership from main to area and then delete the instance after area is executed.
    //println!("The area of the rectangle is {}", area(&rect1));

    //in order to diplay this specfic struct we need to implement std:fmt::Display

    //println!("Here is the rectangle itself {:?}", rect1);// :? is the output format called Debug

    println!("rect1 is {:#?}", &rect1); // need to implement the trait Debug to the struct beofre using :? asa formatter
}
