extern crate utilities;

fn main() {

	let input = utilities::read_file("input.txt");

	// Turn string into array of ints
	let mut v = Vec::new();
	for i in input.chars() {
		v.push(i.to_digit(10).unwrap());
	}
	let ans = calc_captcha(&v, 1);
	println!("{}", ans);

	let v_len = v.len() / 2;
	let ans2 = calc_captcha(&v, v_len);
	println!("{}", ans2);
}

fn calc_captcha(v :&Vec<u32>, lead: usize) -> u32 {
	let mut sum = 0;
	let v_len = v.len();

	// Go through each one
	for (index, value) in v.iter().enumerate() {
		let real_index = (index + lead) % v_len;

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
