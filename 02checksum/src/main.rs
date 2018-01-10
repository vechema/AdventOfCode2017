extern crate utilities;

fn main() {

	let input = utilities::read_file("input.txt");

	// Turn string into array of array of ints
	let mut v = Vec::new();
	for line in input.split("\r\n") {
		let mut line_vec = Vec::new();
		for num in line.split("\t") {
			line_vec.push(num.parse::<u32>().unwrap());
		}
		v.push(line_vec);
	}
	let ans = calc_diff_checksum(&v);
	println!("{}", ans);
}

fn calc_diff_checksum(v :&Vec<Vec<u32>>) -> u32 {

	let mut diffs = Vec::new();

	for line in v.iter() {
		let max = line.iter().max().unwrap();
		let min = line.iter().min().unwrap();
		let diff = max - min;
		diffs.push(diff);
	}

	diffs.iter().sum()
}
