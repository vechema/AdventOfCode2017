extern crate utilities;

fn main() {
    let input = utilities::read_file("input.txt");

	let steps = make_direction_list(&input);
	println!("{:?}",steps);
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

#[derive(Debug)]
enum Direction {
	N,
	NE,
	NW,
	S,
	SE,
	SW,
}
