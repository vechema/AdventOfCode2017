extern crate utilities;

fn main() {

	let input = utilities::read_file("input.txt");

	// Turn string into array of ints
	let mut v = Vec::new();
	for i in input.split("\t") {
		v.push(i.to_string().parse::<u32>().unwrap());
	}
	
	let (num_redist, num_cycles) = num_cycles(&v);
	println!("number redistribution cycles: {}", num_redist);
	println!("how many cycles: {}", num_cycles);
}

fn num_cycles(input: &Vec<u32>) -> (u32, usize) {
	let mut past = Vec::new();
	let mut vec: Vec<u32> = input.clone();
	let mut num_cycles = 0;

	while {
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
		!past.contains(&vec)
	} {}

	let first_index_vec = past.iter().position(|ref s| s == &&vec).unwrap();
	(num_cycles, past.len() - first_index_vec)
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
