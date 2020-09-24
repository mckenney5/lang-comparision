/*
 * rust lang
 * takes the users name and says hello
 * compiles with: rustc rust.rs
 */

use std::io::{self, Write}; // import the standard IO library

fn main(){
	// display prompt
	print!("Enter your name: ");
	io::stdout().flush().unwrap(); // flush the prompt to the screen
	
	// get data and store it in name
	let mut name = String::new();
	io::stdin()
		.read_line(&mut name)
		.expect("Unable to read line"); // error handeling 

	// removes the trailing new line char from input
	name.pop();

	// say hello
	println!("Hello {}!", name);

}

