use std::mem::drop;
use utils::{
    box_t_ref, hello, mybox_t_ref, recursive_type_box, store_int_on_heap, CustomSmartPointer, MyBox,
};

fn main() {
    store_int_on_heap(5);

    recursive_type_box();

    box_t_ref();
    mybox_t_ref();
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
