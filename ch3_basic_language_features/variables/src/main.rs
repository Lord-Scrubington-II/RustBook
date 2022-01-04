#![allow(unused)]

fn main() {
	mutability();
	println!();

	shadowing();
	println!();
}

fn mutability() {
	println!("Mutability demonstration:");
	// x must be declared mutable to be reassigned
	let mut x = 5;
	println!("The value of x is: {}", x); // 5
	x = 6;
	println!("x = 6; \nThe value of x is now: {}", x); // 6

	// constants use ROAD_KILL_CASE. They are valid for the entire time a program runs 
	// within the scope in which they were declared.
	// Constants must have type annotations, cannot be declared mutable, can be declared in any scope (global too),
	// and must be set to a value that can be computed at compile time.
	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}

fn shadowing() {
	println!("Shadowing demonstration:");
	let x = 5;

	let x = x + 1; // x has been shadowed into a new int with value 6
	println!("The value of x is: {}", x); // 6
	{
		let x = x * 2;
		println!("Entered new scope! x = 12; \nThe value of x in the inner scope is: {}", x); // 12
	}

	println!("Returned to original scope! \nThe value of x is: {}", x); // 6

	// shadowing is different from mutation as after a variable is shadowed with `let`, it becomes immutable again.
	// additionally, it allows us to change the type of a variable binding:
	let spaces = "   "; // string
	let spaces = spaces.len(); // i32

	// predictably, we are not allowed to mutate the tye of a mutable variable.
	// this is not allowed:
	/*
	let mut spaces = "   ";
	spaces = spaces.len();
	*/
}