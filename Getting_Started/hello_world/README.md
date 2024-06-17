# Hello World in Rust

After installing Rust, create a folder to store your project. You can do it as follows using command lines:

```shell
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
$ touch main.rs
```

You can now open the main file and write your first function in Rust!
The goal of the program is very simple, and it enables you to be more familiar with Rust syntax.

```rust
fn main() {
    println!("Hello World!");
}
```

Explanations --> [Hello World Program](https://doc.rust-lang.org/book/ch01-02-hello-world.html)

To compile (using rustc COMPILER) and run the programm you can use the following command lines.

```shell
$ rustc main.rs
$ ./main
```

"Hello World !" should be displayed in your terminal.
