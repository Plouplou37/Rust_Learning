fn main() {
    /*let s1 = String::from("Hello");
    let s1_pt = s1.as_ptr();
    let s2 = s1; // Cloning to create s2
    let s2_pt = s2.as_ptr();

    // Print the values and pointers using string interpolation
    println!("s1: {} and s2: {}", s1, s2);
    println!("s1: {:p} and s2: {:p}", s1_pt, s2_pt);*/

    let s = String::from("hello"); // s comes into scope

    let o = takes_ownership(s); // s's value moves into the function...
                                // ... and so is no longer valid here

    println!("I print s here {o}");
    let x = 5; // x comes into scope

    makes_copy(x);

    let s1 = String::from("hello");
    let pt_1 = &s1;
    println!("jjejjeje --> {:p}", pt_1);

    let len: usize = calculate_length_with_ref(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_with_ref(s: &String) -> usize {
    let length = s.len(); // len() returns the length of a String

    length
}

fn takes_ownership(some_string: String) -> String {
    // some_string comes into scope
    some_string
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
