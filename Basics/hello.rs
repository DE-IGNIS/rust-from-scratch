fn main() {
	// This is an example for comments in RUST similar to C/C++

	/*
	And this is an example of
	multi-line comments
	*/

	println!("Hello World this is your beloved spiderman!");

	// let is used for declaring local variables
	let x = 5 * 10 ;

	// {} is a sort of placeholder like ${} in js
	println!("5 * 10 is = {}!",x);

    // Different types of formatting 
	println!("Base 10:               {}",   69420); // 69420
	println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
	println!("Base 8 (octal):        {:o}", 69420); // 207454
	println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
}