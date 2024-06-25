use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blues"), 10);
    scores.insert(String::from("Reds"), 50);

    let team_name = String::from("Blues");

    let kk = match scores.get(&team_name) {
        Some(value) => {
            println!("here is the value {value}");
            value
        }
        None => {
            println!("None value");
            &0 // Provide a default value
        }
    };

    println!("value value {kk}");

    for (key, value) in &scores {
        println!("{key} : {value}");
    }

    /* Adding a key and value only if a key isn't present */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
