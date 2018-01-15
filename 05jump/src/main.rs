extern crate utilities;

fn main() {

	let input = utilities::read_file("input.txt");

	// Turn string into array of ints
	let mut v = Vec::new();
	for i in input.split("\r\n") {
		v.push(i.to_string().parse::<i32>().unwrap());
	}

	println!("{:?}", v);
}
