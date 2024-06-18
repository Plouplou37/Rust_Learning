fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");

    {
        let x: i32 = 10;
        println!("The value of x within the inner scope is {x}");
    }

    println!("The value of x outside the scope is {x}");
}
