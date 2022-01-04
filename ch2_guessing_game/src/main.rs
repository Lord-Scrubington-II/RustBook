#![allow(unused)]
use std::io;

fn main() {
    // print line with line feed
    println!("Guess the number:");

    // variables are IMMUTABLE by default. declare them mutable 
    // using `mut`. 
    let mut guess = String::new(); // String::new() allocates the guess on the `heap`, while literals do not!

    // stdin() is a function! it returns an instance of std::io::Stdin.
    // &mut declares that `guess` is to be passed by reference, and that the reference is mutable.
    // io::Stdin.read_line() returns a `Result`. which is an enum with value `Ok` or `Err`.
    // `expect` crashes the program with the given string as a message if the enum's value is equal to "Err"
    io::stdin().read_line(&mut guess).expect("Failed to read line.");

    // this is the most basic way that string interpolation is performed for println!.
    println!("You guessed: {}", guess);
}

fn print_test() {
    
    // to interpolate multiple strings, the following methods are appropriate:
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);

    // bind with variables
    println!("{b}, {a}?", a="No", b="Yes"); // Yes, No?

    // bind with "index"
    println!("{1}, {0}, {2}.", "vidi", "Veni", "vici"); // Veni, vidi, vici.
}

fn format_test() {

    // to bind an interpolated string, use format!():
    let yes = "Yes";
    let question = format!("{}, no?", yes);
    println!("{}", question); // output: Yes, no?
}