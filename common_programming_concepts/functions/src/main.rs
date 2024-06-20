fn main() {
    let x: i32 = 10;

    println!("The square of {x} is {}", square(x));

    //statement --> do not return value
    //let x = (let y = 6);

    //expression --> evaluate to a value
    //Expression can be part of a statement
    //calling a function is an expression
    //calling a macro is an expression
    //A new scope blok created with {} is an expression

    //This doesnt work because {} doesnt return anything to binnd y with.
    let y = {
        let x = 3;
        x + 1;
    };

    // This work because {} return the value of the expression "x+1"
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");
}

fn square(x: i32) -> i32 {
    x * x
}
