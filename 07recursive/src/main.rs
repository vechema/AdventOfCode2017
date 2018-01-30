extern crate utilities;
use std::collections::{HashSet, HashMap};

fn main() {
	let input = utilities::read_file("input.txt");

	let tower_list = build_tower_list(&input);
	let bottom_tower_name = find_bottom_tower_name(&tower_list);
	println!("{:?}", bottom_tower_name);

	let required_weight = find_problem_child_weight(&tower_list);
	println!("{:?}", required_weight);
}

#[derive(Debug)]
struct Tower {
	name: String,
	weight: u32,
	kids: Vec<String>,
}

fn build_tower_list(input: &String) -> Vec<Tower> {

	let mut result = Vec::new();

	for line in input.split("\r\n") {
		let mut parts = line.split(" ");
		let name = parts.next().unwrap().to_owned();

		let weight_unformatted = parts.next().unwrap();
		let weight = weight_unformatted[1..weight_unformatted.len()-1].parse::<u32>().unwrap();

		let kids = if let Some(_) = parts.next() { // Get rid of ->
			parts.map(|x| {
					if x.contains(",") {
						String::from(&x[..x.len()-1])
					} else {
						String::from(x)
					}
				})
				.collect::<Vec<String>>()
		} else {
			Vec::new()
		};

		let new_tower = Tower {name, weight, kids};
		result.push(new_tower);
	}

	result
}

fn find_bottom_tower_name(tower_list: &Vec<Tower>) -> &String {
	let mut all_the_chillans: HashSet<&String> = HashSet::new();
	for &Tower{ref kids, ..} in tower_list {
		for kid in kids {
			all_the_chillans.insert(kid);
		}
	}

	let mut all_tower_names: HashSet<&String> = HashSet::new();
	for tower in tower_list {
		let &Tower {ref name, ..} = tower;
		all_tower_names.insert(name);
	}

	all_tower_names.difference(&all_the_chillans).next().unwrap()
}

fn find_problem_child_weight(tower_list: &Vec<Tower>) -> u32 {

	4
}
