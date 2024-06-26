use std::cmp::PartialOrd;

mod point;
use point::Point;

mod point2;
use point2::Point as Point2;

/*fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// We need to specify that the function is using generic in the signature of the function.

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}*/

fn main() {
    /*let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = Point::new(0, 10);
    println!("{}", integer);

    let float = Point::new(0.5, 10.5);
    println!("{:#?}", float);

    println!(
        "float distance from origin is {}",
        float.distance_from_origin()
    );*/

    let p1 = Point2::new(98, 'c');
    println!("p1 is {:#?}", p1);

    let p2 = Point2::new(0, 0.5);
    println!("p2 is {:#?}", p2);

    let p3 = p2.mixup(p1);

    println!("p3 is {:#?}", p3);
}
