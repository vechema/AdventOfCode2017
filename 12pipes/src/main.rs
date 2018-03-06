extern crate utilities;

fn main() {
    let input = utilities::read_file("input.txt");
	let edge_list = format_input(&input);
	let group_member = 0;
	let group = find_group(group_member, &edge_list);
	println!("Group for {}, {:?}. Length: {}",group_member, group, group.len());

	let all_groups = find_all_groups(&edge_list);
	println!("All groups: {:?}. Length: {}", all_groups, all_groups.len());

}

fn format_input(input: &String) -> Vec<Vec<usize>> {
	let mut result : Vec<Vec<usize>> = Vec::with_capacity(2000);
	for line in input.lines() {
		let mut parts = line.split(" ");
		let index = parts.next().unwrap().parse::<usize>().unwrap();

		// Get rid of <->
		parts.next();

		let kids = {
			parts.map(|x| { // &str
					if x.contains(",") {
						(&x[..x.len()-1]).parse::<usize>().unwrap()
					} else {
						x.parse::<usize>().unwrap()
					}
				})
			.collect::<Vec<usize>>()
		};
		result.push(kids);
	}

	result
}

fn find_group(member: usize, edge_list: &Vec<Vec<usize>>) -> Vec<usize>{
	let mut index_list : Vec<usize> = vec![member];
	let mut used_indices = Vec::new();

	while !index_list.is_empty() {
		let index = index_list.pop().unwrap();
		if !used_indices.contains(&index) {
			used_indices.push(index);

			// Add all the elements of index to index_list
			let mut useable_edges = edge_list[index].clone();
			index_list.append(&mut useable_edges);
		}
	}
	used_indices
}

fn find_all_groups(edge_list: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
	let num_edges = edge_list.len();
	let mut indices = (0..num_edges).collect::<Vec<usize>>();
	let mut result = Vec::new();

	for ind in 0..num_edges {
		if indices.contains(&ind) {
			let ind_group = find_group(ind, &edge_list);
			result.push(ind_group.clone());
			indices = indices.into_iter().filter(|e| !ind_group.contains(e)).collect::<Vec<usize>>();
		}
	}

	result
}