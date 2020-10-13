/*
 * rust lang
 * takes the users name and says hello
 * compiles with: rustc rust.rs
 */

// import the standard IO library
use std::io::{self, Write}; 

fn main(){
	// display prompt
	print!("Enter your name: ");

	// flush the prompt to the screen
	io::stdout().flush().unwrap(); 	

	// get data and store it in name
	let mut name = String::new();
	io::stdin()
		.read_line(&mut name)
		.expect("Unable to read line"); 
		// error handeling 

	// removes the trailing new line char
	// from input
	name.pop();

	// say hello
	println!("Hello {}!", name);

}

