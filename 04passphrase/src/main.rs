extern crate utilities;
use std::collections::HashSet;

fn main() {

	let input = utilities::read_file("input.txt");
	// Turn string into array of array of strings
	let mut v = Vec::new();
	for line in input.split("\r\n") {
		let mut line_vec = Vec::new();
		for passphrase in line.split(" ") {
			line_vec.push(passphrase);
		}
		v.push(line_vec);
	}
	let ans = calc_num_passphrases(&v);
	println!("num passphrases: {}", ans);
}

fn calc_num_passphrases(v :&Vec<Vec<&str>>) -> u32 {
	let mut sum = 0;
	for line in v.iter() {
		let mut unique = false;
		let mut set = HashSet::new();
		for word in line.iter() {
			unique = set.insert(word);
			if !unique {
				break;
			}
		}
		if unique {
			sum += 1;
		}
	}
	sum
}