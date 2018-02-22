extern crate utilities;
use self::Direction::{N,NE,NW,S,SE,SW};
use self::ComboDirection::{Cancel,Combo,NoCombo};

fn main() {
    let input = utilities::read_file("input.txt");

	let steps = make_direction_list(&input);
	let compact_steps = compact_directions(&steps);
	println!("{:?}", compact_steps);
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
	Vec::new()
}

fn combine_directions(direction1: &Direction, direction2: &Direction) -> ComboDirection {
	if are_opposite(direction1, direction2) {
		return Cancel;
	}

	if let Combo(i) = combo_direction(direction1, direction2) {
		return Combo(i);
	}

	match (direction1, direction2) {
		(&N, _) | (_, &N) => return Combo(N),
		(_, _) => return NoCombo,
	}

	NoCombo
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
			return(Combo(NW));
		} else if direction2.is_east() {
			return(Combo(NE));
		}
	}
	NoCombo
}

fn combo_direction_south(direction1: &Direction, direction2: &Direction) -> ComboDirection {

	if direction1 == &S && direction2.is_north() {
		if direction2.is_west() {
			return(Combo(SW));
		} else if direction2.is_east() {
			return(Combo(SE));
		}
	}
	NoCombo
}

#[derive(Debug,Eq,PartialEq)]
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
}
