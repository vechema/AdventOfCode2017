extern crate utilities;

fn main() {

	let input = utilities::read_file("input.txt");

	// Turn string into array of ints
	let mut v = Vec::new();
	for i in input.split("\t") {
		v.push(i.to_string().parse::<u32>().unwrap());
	}
	
	let ans = num_cycles(&v);
	println!("{}",ans);
}

fn num_cycles(input: &Vec<u32>) -> u32 {
	let mut past = Vec::new();
	let mut vec: Vec<u32> = input.clone();
	let mut num_cycles = 0;

	while !past.contains(&vec) {
		past.push(vec.clone());

		let mut max: u32 = *vec.iter().max().unwrap();
		let mut max_index: usize = vec.iter().position(|&s| s == max).unwrap();

		let mut new_vec = vec.clone();
		new_vec[max_index] = 0;
		while max > 0 {
			max_index = (max_index + 1) % vec.len();
			new_vec[max_index] = new_vec[max_index] + 1;
			max -= 1;
		}
		vec = new_vec;
		num_cycles +=1;
	}
	num_cycles
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn captcha_simple_repeat() {
		let vec = vec![0, 2, 7, 0];
		assert_eq!(num_cycles(&vec), 5);
	}
}
