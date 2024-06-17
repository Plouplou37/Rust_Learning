Writing more complex code you'll have to manage dependencies.

CARGO is the package manager by default for rust application.

- building your code,
- downloading libraries you code depends on,
- building those libraries/ dependencies.

Check version of cargo

```shell
$ cargo --version
```

Creating a new project using cargo.

```shell
$ cargo new <name_of_the_project>
$ cd <name_of_the_project>
```

A "src" folder and a Cargo.toml file and a src/main.rs file are created.

Push to Github repository:

```shell
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

Explanations --> [Hello Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
