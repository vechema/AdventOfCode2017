extern crate utilities;

fn main() {

	let input = utilities::read_file("input.txt");

	// Turn string into array of ints
	let mut v = Vec::new();
	for i in input.split("\t") {
		v.push(i.to_string().parse::<u32>().unwrap());
	}
	
	println!("{:?}",v);
}