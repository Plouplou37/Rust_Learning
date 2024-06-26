//take a reference of the vector and return a reference to the largest value in the vector.
fn largest(list: &Vec<i32>) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("{number_list:?}");

    let result = largest(&number_list);
    println!("{number_list:?}");

    println!("The largest number is {result:#?}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {result:#?}");
}
