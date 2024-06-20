fn main() {
    /*let mut s1 = String::from("hello");

    let pt1: &mut String = &mut s1;
    pt1.push_str("pt1push");
    let len: usize = compute_str_len(pt1);
    // /println!("ot1 {pt1}");
    let pt2: &mut String = &mut s1;
    pt2.push_str("pt2 push");
    println!("s1 value is {s1} and length is {len}");
    */

    /*let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    println!("{s1}");*/

    let p: String = dangle();

    println!("the reference to p is {:p}", &p);
}

fn compute_str_len(text: &mut String) -> usize {
    // IMPORTANT --> text should normally be a pointer only, however rust automatically "Dereference" it.
    text.push_str("new push");
    println!("text is {}", text);
    text.len()
}

fn dangle() -> String {
    let s = String::from("Hello");
    println!("The reference to s in the function is {:p}", &s);
    s
}
