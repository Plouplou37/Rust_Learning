pub fn reverse(input: &str) -> String {
    //chain methods, get each chars and reverse
    input.chars().rev().collect()
}

fn main() {
    let name = "Baven";

    let reverse = reverse(name);

    println!("name is {:#?}", reverse);
    /*let mut v: Vector<i32> = Vec::new();

    v.psuh(1);
    v.push(2);
    v.push(3);*/

    /*let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let third: i32 = v[2];

    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    //let does_not_exist = v.get(100);*/

    let mut v = vec![1, 2, 3, 4, 5];
    println!("{}", v[0]);
    let first = &v[0];

    use_first(first);

    //v.push(6);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

pub fn use_first(first: &i32) {
    println!("{first}");
}
