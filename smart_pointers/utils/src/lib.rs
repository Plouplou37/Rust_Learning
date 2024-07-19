use crate::List::{Cons, Nil};
use std::ops::Deref;

pub fn store_int_on_heap(x: i32) {
    let b = Box::new(x);
    println!("b is {b}");
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn recursive_type_box() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

pub fn box_t_ref() {
    let x = 5;
    let y = &x;
    let y_box = Box::new(x);
    println!("x is {x}");

    println!("y is {:p} and y_box is {:p}", y, y_box);
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

pub fn mybox_t_ref() {
    let x = 10;
    let y = &x;
    let y_box = MyBox::new(x);
    println!("x is {x}");

    println!("y is {} and y_box is {}", y, *y_box);
}

pub fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data  `{}`!", self.data);
    }
}
