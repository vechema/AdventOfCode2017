extern crate utilities;

fn main() {
    let input = utilities::read_file("input.txt");

	let mut lengthes = Vec::new();
    for num in input.split(",") {
		lengthes.push(num.parse::<usize>().unwrap());
	}

	let knot_hash_vec = knot_hash(&lengthes);
	let knot_hash = knot_hash_vec[0] * knot_hash_vec[1];
	println!("{}",knot_hash);
}

fn knot_hash(lengthes: &Vec<usize>) -> Vec<u32> {
	let mut list: Vec<u32> = (0..256).collect();

	let mut position: usize = 0;
	let mut skip_size = 0;
	for len in lengthes {
		let mut list_slice = list_slice(&list, position, *len);
		list_slice.reverse();
		insert_slice(&mut list, &list_slice, position);
		position = (position + len + skip_size) % list.len();
		skip_size+=1;
	}

	list
}

fn list_slice(list: &Vec<u32>, position: usize, length: usize) -> Vec<u32> {
	if position + length < list.len() {
		return list[position..position+length].to_vec();
	}

	let final_index = (position + length) % list.len();
	let mut list_start = list[position..list.len()].to_vec();
	let mut list_end = list[0..final_index].to_vec();

	list_start.append(&mut list_end);

	list_start
}

fn insert_slice(list: &mut Vec<u32>, slice: &Vec<u32>, position: usize) {
	if position + slice.len() < list.len() {
		let mut slice_index = 0;
		for index in position..position+slice.len() {
			list[index] = slice[slice_index];
			slice_index+=1;
		}
	} else {

		let mut slice_index = 0;
		for index in position..list.len() {
			list[index] = slice[slice_index];
			slice_index+=1;
		}

		let final_index = (position + slice.len()) % list.len();
		for index in 0..final_index {
			list[index] = slice[slice_index];
			slice_index+=1;
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn list_slice_wrap() {
		let list = vec![0,1,2,3,4];
		assert_eq!(list_slice(&list,3,4), vec![3,4,0,1]);
	}

	#[test]
	fn insert_slice_basic() {
		let mut list = vec![0,1,2,3,4];
		let slice = vec![7,8,9];
		insert_slice(&mut list, &slice, 1);
		assert_eq!(list, vec![0,7,8,9,4]);
	}

	#[test]
	fn insert_slice_wrap() {
		let mut list = vec![0,1,2,3,4];
		let slice = vec![7,8,9,10];
		insert_slice(&mut list, &slice, 3);
		assert_eq!(list, vec![9,10,2,7,8]);
	}
}
