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
	let ans = calc_num_passphrases(&v, false);
	println!("num passphrases: {}", ans);

	let ans2 = calc_num_passphrases(&v, true);
	println!("num passphrases: {}", ans2);
}

fn calc_num_passphrases(v :&Vec<Vec<&str>>, strict: bool) -> u32 {
	let mut sum = 0;
	for line in v.iter() {
		let mut unique = false;
		let mut set = HashSet::new();
		for word in line.iter() {
			let mut str_word = String::from(*word);
			if strict {
				let mut chars: Vec<char> = word.chars().collect();
				chars.sort();
				str_word = chars.into_iter().collect::<String>();
			}
			unique = set.insert(str_word);
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