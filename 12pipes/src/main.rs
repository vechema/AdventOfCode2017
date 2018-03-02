extern crate utilities;
fn main() {
    let input = utilities::read_file("input.txt");
	let edge_list = format_input(&input);
	println!("{:?}",edge_list);
	let group = find_group(0, &edge_list);
}

fn format_input(input: &String) -> Vec<Vec<i32>> {
	let mut result : Vec<Vec<i32>> = Vec::with_capacity(2000);
	for line in input.lines() {
		let mut parts = line.split(" ");
		let index = parts.next().unwrap().parse::<usize>().unwrap();

		// Get rid of <->
		parts.next();

		let kids = {
			parts.map(|x| { // &str
					if x.contains(",") {
						(&x[..x.len()-1]).parse::<i32>().unwrap()
					} else {
						x.parse::<i32>().unwrap()
					}
				})
			.collect::<Vec<i32>>()
		};
		result.push(kids);
	}

	result
}

fn find_group(member: usize, edge_list: &Vec<Vec<i32>>) -> Vec<i32>{
	Vec::new()
}