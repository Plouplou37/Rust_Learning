fn main() {
    //panic!("crash and burn");

    // can use "RUST_BACKTRACE=1 cargo run" in command line to get the backtrace.
    let v = vec![1, 2, 3];

    v[99];
}
