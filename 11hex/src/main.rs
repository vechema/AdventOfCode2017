extern crate utilities;
use self::Direction::{N,NE,NW,S,SE,SW};
use self::ComboDirection::{Cancel,Combo,NoCombo};

fn main() {
    let input = utilities::read_file("input.txt");

	let steps = make_direction_list(&input);
	let compact_steps = compact_directions(&steps);
	println!("{:?}, length: {}", compact_steps, compact_steps.len());
}

fn make_direction_list(input: &String) -> Vec<Direction> {

	let mut steps = Vec::new();
	for dir in input.split(",") {
		let direction = match dir {
            "n"  => Direction::N,
            "ne" => Direction::NE,
            "nw" => Direction::NW,
            "s"  => Direction::S,
            "se" => Direction::SE,
			_    => Direction::SW,
        };
		steps.push(direction);
	}

	steps
}

fn compact_directions(directions: &Vec<Direction>) -> Vec<Direction> {
	let mut result = Vec::new();
	let mut temp_directions: Vec<Direction> = directions.clone();
	let mut to_remove : Vec<usize> = vec![0];

	while to_remove.len() > 0{
		//println!("directions length: {}", temp_directions.len());
		to_remove = Vec::new();
		for (i1, dir1) in temp_directions.iter().enumerate() {
			for (i2, dir2) in temp_directions.iter().enumerate().skip(i1) {
				//println!("{:?}({}), {:?}({})", dir1,i1,dir2,i2);
				if !to_remove.contains(&i1) && !to_remove.contains(&i2) {
					match combine_directions(&dir1, &dir2) {
						Cancel => {to_remove.push(i1); to_remove.push(i2);},
						NoCombo => {},
						Combo(i) => {
							to_remove.push(i1);
							to_remove.push(i2);
							result.push(i);
						},
					}
				} else {
					//println!("\t{} or {} has been removed", i1, i2);
					//println!("\t{:?}", to_remove);
				}
			}
		}
		to_remove.sort_by(|a, b| b.cmp(a));
		for remove in to_remove.iter() {
			temp_directions.remove(*remove);
		}

		result.append(&mut temp_directions);
		//println!("result: {:?}", result);
		if to_remove.len() > 0 {
			temp_directions = result.clone();
			result = Vec::new();
		}
	}

	result
}

fn combine_directions(direction1: &Direction, direction2: &Direction) -> ComboDirection {
	if are_opposite(direction1, direction2) {
		return Cancel;
	}

	if let Combo(i) = combo_direction(direction1, direction2) {
		return Combo(i);
	}

	match (direction1, direction2) {
		(&NE, &NW) | (&NW, &NE) => return Combo(N),
		(&SE, &SW) | (&SW, &SE) => return Combo(S),
		(_, _) => return NoCombo,
	}
}

fn are_opposite(dir1: &Direction, dir2: &Direction) -> bool {
	// N&S
	// NE&SW
	// NW&SE
	match (dir1, dir2) {
		(&N, &S) => return true,
		(&S, &N) => return true,
		(&NE, &SW) => return true,
		(&SW, &NE) => return true,
		(&NW, &SE) => return true,
		(&SE, &NW) => return true,
		(_, _) => return false,
	}
}

fn combo_direction(dir1: &Direction, dir2: &Direction) -> ComboDirection {

	if let Combo(i) = combo_direction_north(dir1,dir2) {
		return Combo(i);
	} else if let Combo(i) = combo_direction_north(dir2,dir1) {
		return Combo(i);
	} else if let Combo(i) = combo_direction_south(dir1,dir2) {
		return Combo(i);
	} else if let Combo(i) = combo_direction_south(dir2,dir1) {
		return Combo(i);
	}

	NoCombo
}

fn combo_direction_north(direction1: &Direction, direction2: &Direction) -> ComboDirection {

	if direction1 == &N && direction2.is_south() {
		if direction2.is_west() {
			return Combo(NW);
		} else if direction2.is_east() {
			return Combo(NE);
		}
	}
	NoCombo
}

fn combo_direction_south(direction1: &Direction, direction2: &Direction) -> ComboDirection {

	if direction1 == &S && direction2.is_north() {
		if direction2.is_west() {
			return Combo(SW);
		} else if direction2.is_east() {
			return Combo(SE);
		}
	}
	NoCombo
}

#[derive(Debug,Eq,PartialEq,Clone)]
enum Direction {
	N,
	NE,
	NW,
	S,
	SE,
	SW,
}

#[derive(Debug,Eq,PartialEq)]
enum ComboDirection {
	Cancel,
	Combo(Direction),
	NoCombo,
}

impl Direction {
	fn is_north(&self) -> bool {
		match self {
			&N | &NE | &NW => true,
			_ => false,
		}
	}

	fn is_south(&self) -> bool {
		match self {
			&S | &SE | &SW => true,
			_ => false,
		}
	}

	fn is_east(&self) -> bool {
		match self {
			&NE | &SE => true,
			_ => false,
		}
	}

	fn is_west(&self) -> bool {
		match self {
			&NW | &SW => true,
			_ => false,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn are_opposite_true() {
		assert_eq!(are_opposite(&N,&S),true);
	}

	#[test]
	fn are_opposite_false() {
		assert_eq!(are_opposite(&NE,&S),false);
	}

	#[test]
	fn combo_opposites() {
		assert_eq!(combine_directions(&N,&S),Cancel);
	}

	#[test]
	fn combo_combo0() {
		assert_eq!(combine_directions(&N,&SW),Combo(NW));
	}

	#[test]
	fn combo_combo1() {
		assert_eq!(combine_directions(&SW,&N),Combo(NW));
	}

	#[test]
	fn combo_combo2() {
		assert_eq!(combine_directions(&NE,&S),Combo(SE));
	}

	#[test]
	fn combo_combo3() {
		assert_eq!(combine_directions(&S,&NE),Combo(SE));
	}

	#[test]
	fn combo_combo4() {
		assert_eq!(combine_directions(&SW,&SE),Combo(S));
	}

	#[test]
	fn combo_combo5() {
		assert_eq!(combine_directions(&N,&SE),Combo(NE));
	}

	#[test]
	fn combo_no_combo() {
		assert_eq!(combine_directions(&SW,&S), NoCombo);
	}

	#[test]
	fn combo_cancel() {
		assert_eq!(combine_directions(&NE,&SW),Cancel);
	}

	#[test]
	fn combo_list0() {
		assert_eq!(compact_directions(&vec![N,NE,NW,S,S]),vec![]);
	}

	#[test]
	fn combo_list1() {
		assert_eq!(compact_directions(&vec![NE,NE,NE]),vec![NE,NE,NE]);
	}

	#[test]
	fn combo_list2() {
		assert_eq!(compact_directions(&vec![NE,NE,SW,SW]),vec![]);
	}

	#[test]
	fn combo_list3() {
		assert_eq!(compact_directions(&vec![NE,NE,S,S]),vec![SE,SE]);
	}

	#[test]
	fn combo_list4() {
		assert_eq!(compact_directions(&vec![SE,SW,SE,SW,SW]),vec![S,S,SW]);
	}

	#[test]
	fn combo_list5() {
		assert_eq!(compact_directions(&vec![SE,SW,SE,SW,SW]),vec![S,S,SW]);
	}
}
