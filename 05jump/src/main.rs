extern crate utilities;

fn main() {

	let input = utilities::read_file("input.txt");

	// Turn string into array of ints
	let mut v = Vec::new();
	for i in input.split("\r\n") {
		v.push(i.to_string().parse::<i32>().unwrap());
	}

	let ans = num_steps(&v, <i32>::max_value());
	println!("{:?}", ans);

	let ans2 = num_steps(&v, 3);
	println!("{:?}", ans2);
}

fn num_steps(vec: &Vec<i32>, decrease_threshold: i32) -> u32 {
	let mut input = vec.clone();
	let mut num_steps = 0;
	let mut index = 0;
	while index < input.len() {
		let jump = input[index];
		if jump < decrease_threshold {
			input[index] = jump + 1;
		} else {
			input[index] = jump - 1;
		}
		index = (index as i32 + jump) as usize;
		num_steps+=1;
	}
	num_steps
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn captcha_simple_repeat() {
		let vec = vec![0, 3, 0, 1, -3];
		assert_eq!(num_steps(&vec), 5);
	}
}