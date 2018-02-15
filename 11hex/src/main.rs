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
	if areOpposite(direction1, direction2) {
		return Cancel;
	}
	NoCombo
}

fn areOpposite(dir1: &Direction, dir2: &Direction) -> bool {
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

#[derive(Debug)]
enum Direction {
	N,
	NE,
	NW,
	S,
	SE,
	SW,
}

#[derive(Debug)]
enum ComboDirection {
	Cancel,
	Combo(Direction),
	NoCombo,
}

impl Direction {
	fn isNorth(&self) -> bool {
		match self {
			&N | &NE | &NW => true,
			_ => false,
		}
	}

	fn isSouth(&self) -> bool {
		match self {
			&S | &SE | &SW => true,
			_ => false,
		}
	}

	fn isEast(&self) -> bool {
		match self {
			&NE | &SE => true,
			_ => false,
		}
	}

	fn isWest(&self) -> bool {
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
		assert_eq!(areOpposite(&N,&S),true);
	}

	#[test]
	fn are_opposite_false() {
		assert_eq!(areOpposite(&NE,&S),false);
	}
}
