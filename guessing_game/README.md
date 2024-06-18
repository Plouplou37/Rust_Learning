# Porgramming a Guessing Game

This Chapter aims to introduce few common Rust concepts through hand-on exercise.

## classic beginner programming problem: a guessing game

Here’s how it works. The program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.

## Setting Up a new project using CARGO !

```shell
$ cargo new guessing_game
$ cd guessing_game
```

Explanations --> [Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

Key concepts:

- match function to handle error, can also behave like switch case structure in C --> similar to try:..except:.. block in python
- some function return type Result. Result is an enum which can take two states, Ok() or Err()
- crate: libraries (bunch of rust files)
- expect() enable to catch-up errors and raised them.
- reference enable to acces value of a variable in multiple piece of the code.
