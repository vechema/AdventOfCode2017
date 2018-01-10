use std::fs::File;
use std::io::prelude::*;

fn main() {
	// Read in file
	let filename = "input.txt";
	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");
		
	println!("{}", contents);
}
