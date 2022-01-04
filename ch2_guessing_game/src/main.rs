#![allow(unused)]
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Welcome to 'Guess the number'.");

	// generate a random number between 1 and 100, inclusive.
	// thread_rng() gives us an instance of a RNG that is local to the currently executing thread
	let secret_number = rand::thread_rng().gen_range(1..=100);

	loop {
		// print line with line feed
		println!("Guess the number (1-100):");

		// variables are IMMUTABLE by default. declare them mutable using `mut`. 
		let mut guess = String::new(); // String::new() allocates the guess on the `heap`, while literals do not!

		// stdin() is a function! it returns an instance of std::io::Stdin.
		// &mut declares that `guess` is to be passed by reference, and that the reference is mutable.
		// io::Stdin.read_line() returns a `Result`. which is an enum with value `Ok` or `Err`.
		// `expect` crashes the program with the given string as a message if the enum's value is equal to "Err"
		io::stdin().read_line(&mut guess).expect("Failed to read line.");

		// produce an unsigned 32 bit integer from the guess.
		// we can *shadow* the original variable and redeclare it as a different type!
		let guess: u32 = match guess.trim().parse() {
			// we can match on any routine that returns an enum of Result type to handle errors.
			Ok(num) => num,
			Err(_) => continue,
		};

		// this is the most basic way that string interpolation is performed for println!.
		println!("You guessed: {}", guess);

		// we can define concise branch-on-return values with the `match` keyword.
		// it can be thought of as a compact switch-case.
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("That number is too small."),
			Ordering::Greater => println!("That number is too large."),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
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