use std::fs::File;
use std::io::prelude::*;

pub fn read_file(filename: &str) -> String {
	// Read in file
	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	contents
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_file_test() {


    }
}
