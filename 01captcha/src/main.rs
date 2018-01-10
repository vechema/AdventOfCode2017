use std::fs::File;
use std::io::prelude::*;

fn main() {
	// Read in file
	let filename = "input.txt";
	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	// Turn string into array of ints
	let mut v = Vec::new();
	for i in contents.chars() {
		v.push(i.to_digit(10).unwrap());
	}
	let ans = calc_captcha(&v, 1);
	println!("{}", ans);

	let v_len = v.len() as u32 / 2;
	let ans2 = calc_captcha(&v, v_len);
	println!("{}", ans2);
}

fn calc_captcha(v :&Vec<u32>, lead: u32) -> u32 {
	let mut sum = 0;
	let v_len = v.len() as u32;

	// Go through each one
	for (index, value) in v.iter().enumerate() {
		let real_index = (( index as u32 + lead) % v_len) as usize;

		if *value == v[real_index] {
			sum+=*value;
		}
	}

	sum
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn captcha_simple_repeat() {
		let vec = vec![1, 1, 2, 2];
		assert_eq!(calc_captcha(&vec,1), 3);
	}

	#[test]
	fn captcha_all_repeat() {
		let vec = vec![1, 1, 1, 1];
		assert_eq!(calc_captcha(&vec,1), 4);
	}

	#[test]
	fn captcha_no_repeat() {
		let vec = vec![1, 2, 3, 4];
		assert_eq!(calc_captcha(&vec,1), 0);
	}

	#[test]
	fn captcha_circular_repeat() {
		let vec = vec![9,1,2,1,2,1,2,9];
		assert_eq!(calc_captcha(&vec,1), 9);
	}
}
