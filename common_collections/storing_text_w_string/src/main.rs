fn main() {
    let data = "Initial Contents";

    let mut s = String::from(data);

    s.push_str("bar");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let r = format!("{s1}-{s2}-{s3}");
    println!("r is {r}");

    println!("{s1} {s2} {s3}");

    for letter in "Зд".chars() {
        println!("{letter}");
    }

    for byte in "Зд".bytes() {
        println!("{byte}");
    }

    let mut bananas = "bananas old";
    assert!(bananas.contains("nana"));
    //assert!(bananas.contains("apples"));

    let result = bananas.replace("old", "new");
    println!("{result}");
}
