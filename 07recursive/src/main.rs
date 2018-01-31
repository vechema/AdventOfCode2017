extern crate utilities;
use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;

fn main() {
	let input = utilities::read_file("input.txt");

	let tower_map = build_tower_list(&input);
	let bottom_tower_name = find_bottom_tower_name(&tower_map);
	println!("{:?}", bottom_tower_name);

	let required_weight = find_problem_child_weight(&tower_map);
	println!("{:?}", required_weight);
}

#[derive(Debug)]
struct Tower {
	name: String,
	weight: u32,
	kids: Vec<String>,
}

fn build_tower_list(input: &String) -> HashMap<String, Tower> {

	let mut result = HashMap::new();

	for line in input.lines() {
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

		let new_tower = Tower {name: name.clone(), weight, kids};
		result.insert(name.clone(), new_tower);
	}

	result
}

fn find_bottom_tower_name(tower_map: &HashMap<String, Tower>) -> &String {
	let mut all_the_chillans: HashSet<&String> = HashSet::new();
	for &Tower{ref kids, ..} in tower_map.values() {
		for kid in kids {
			all_the_chillans.insert(kid);
		}
	}

	let mut all_tower_names: HashSet<&String> = HashSet::new();
	for name in tower_map.keys() {
		all_tower_names.insert(name);
	}

	all_tower_names.difference(&all_the_chillans).next().unwrap()
}

fn find_problem_child_weight(tower_map: &HashMap<String, Tower>) -> u64 {

	let bottom_tower_name = find_bottom_tower_name(&tower_map);
	get_children_weight(&tower_map, bottom_tower_name, 0)
}

fn get_children_weight(tower_map: &HashMap<String, Tower>, tower_name: &String, current_weight: u64) -> u64 {
	let mut sum_weight = current_weight;
	let tower_entry = tower_map.get(tower_name).unwrap();

	let mut kid_weights = HashMap::new();

	for kid in tower_entry.kids.iter() {
		let kid_weight = tower_map.get(kid).unwrap().weight as u64;
		let sum_kids_weight = get_children_weight(tower_map, kid, kid_weight);

		match kid_weights.entry(sum_kids_weight) {
            Entry::Vacant(e) => { e.insert(vec![kid]); },
            Entry::Occupied(mut e) => { e.get_mut().push(kid); }
        }
		sum_weight+= sum_kids_weight;
	}

	if tower_entry.kids.len() > 0 {
		if kid_weights.len() > 1 {
			let off_weight_parent = kid_weights.values().find(|x| x.len() == 1).unwrap()[0];
			let mut kid_weights_keys = kid_weights.keys();
			let weight_one = kid_weights_keys.next().unwrap();
			let weight_two = kid_weights_keys.next().unwrap();
			let mut weight_diff: i64 = *weight_one as i64 - *weight_two as i64;
			if kid_weights.get(weight_one) == Some(&vec![off_weight_parent]) {
				weight_diff = *weight_two as i64 - *weight_one as i64;
			}
			let off_weight_parent_weight = tower_map.get(off_weight_parent).unwrap().weight;
			println!("{:?} off by {}, change weight from {} to {}", off_weight_parent, weight_diff, off_weight_parent_weight, off_weight_parent_weight as i64 + weight_diff);
		}
	}
	sum_weight
}