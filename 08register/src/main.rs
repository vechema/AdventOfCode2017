extern crate utilities;

fn main() {
	let input = utilities::read_file("input.txt");
	
	for instruction in input.lines() {
		println!("{}", instruction);
	}
}
