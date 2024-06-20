fn main() {
    let mut s = String::from("hello world");
    /*
    //We only borrow s but do not change ownership so s will not be "FREE" after execution of the fn first_word(...)
    let word = first_word(&s);
    println!("the first word is here {word}");
    s.clear();
    //println!("the value of s is here {s}");*/

    //referenced slice which contain pointer of the begin and the len ! so here point -> [0] to [+len] with len = 5
    /*let hello = &s[0..5];

    let world = &s[5..s.len()];*/

    //first_word is now returning a reference to a slice of s, so nothing can be done while the reference is deleted.
    let word = first_word(&s);
    let l = word.len();
    println!("the first word is here {word} of size {l}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
