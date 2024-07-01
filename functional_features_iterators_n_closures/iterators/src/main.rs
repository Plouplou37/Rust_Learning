fn main() {
    let v1 = vec![1, 2, 3];

    //iterator are lazy, they only take effect when called !
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("{val}");
    }

    //println!("{:#?}", v1_iter); // can't because the for loop take ownership of v1_iter ans consume it !

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("{:#?}", v2);
}
/*
#[test]
fn iterator_demonstrating() {
    let v1 = vec![1, 2, 3];

    // .next() on iterator change the internal state of the Iterator, it must then be declared as mutable !
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn map_test() {
    let v1: Vec<i32> = vec![1, 2, 3];

    //Note --> map() take a closure, so we can put as many operation as we want in the closure to transform each x separetely.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}*/
