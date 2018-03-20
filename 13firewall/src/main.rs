extern crate utilities;
use std::fmt;

fn main() {
    let input = utilities::read_file("input.txt");
	let scanners = create_scanners(&input);
	//println!("{:?}",scanners);

	let score = run(&scanners);
	println!("{}", score);
}

fn create_scanners(input: &String) -> Vec<Scanner> {
	let mut result = Vec::new();
	let mut index = 0;
	for line in input.lines() {
		let mut parts = line.split(" ");

		let raw_layer = parts.next().unwrap();
		let depth = (&raw_layer[..raw_layer.len()-1]).parse::<u32>().unwrap();
		while index < depth {
			result.push(Scanner::new(index,0));
			index += 1;
		}
		index += 1;

		let range = parts.next().unwrap().parse::<u32>().unwrap();
		result.push(Scanner::new(depth,range));
	}

	result
}

fn run(scanners: &Vec<Scanner>) -> u32 {
	let mut score = 0;
	let mut scanners_mut = scanners.clone();

	for player_position in 0..scanners_mut.len()
	{
		{
			// First move the player
			let current_scanner = &mut scanners_mut[player_position];
			current_scanner.has_player = true;

			// Check if there's a guard there, add to score
			if current_scanner.position == 0 {
				score = score + current_scanner.depth * current_scanner.range;
			}
		}

		{
			if player_position > 0 {
				let previous_scanner = &mut scanners_mut[player_position-1];
				previous_scanner.has_player = false;
			}
		}

		// Then move the scanners
		for scanner in scanners_mut.iter_mut() {
			scanner.scan();
		}
	}

	score
}

#[derive(Clone)]
struct Scanner {
	depth: u32,
	range: u32,
	position: u32,
	going_down: bool,
	has_player: bool,
}

impl Scanner {

	fn new(depth: u32, range: u32) -> Scanner {
		Scanner {depth: depth, range: range, position: 0, going_down: true, has_player: false}
	}

	fn scan(&mut self) -> u32 {
		let mut new_pos = 0;
		if self.range == 0 {
			return 0;
		}
		if self.going_down && self.position < self.range - 1 {
			new_pos = self.position + 1
		} else if self.position == self.range - 1 {
			self.going_down = false;
			new_pos = self.position - 1;
		} else if !self.going_down && self.position > 0 {
			new_pos = self.position - 1;
		} else if self.position == 0 {
			self.going_down = true;
			new_pos = self.position + 1;
		}

		self.position = new_pos;
		new_pos
	}
}

impl fmt::Debug for Scanner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.range == 0 {
			if self.has_player {
				write!(f, "{}: (.)", self.depth)?;
			} else {
				write!(f, "{}: ...", self.depth)?;
			}
			write!(f, "\n")
		} else {
			write!(f, "{}: ", self.depth)?;
			for x in 0..self.range {
				if x == self.position {
					if self.has_player && x == 0 {
						write!(f, "(S)")?;
					} else {
						write!(f, "[S]")?;
					}
				} else {
					if self.has_player && x == 0 {
						write!(f, "( )")?;
					} else {
						write!(f, "[ ]")?;
					}
				}
			}
			write!(f, "\n")
		}
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn scan_once() {
		let mut scanner = Scanner::new(1,4);
		assert_eq!(scanner.scan(), 1);
	}

	#[test]
	fn scan_twice() {
		let mut scanner = Scanner::new(1,4);
		scanner.scan();
		assert_eq!(scanner.scan(), 2);
	}

	#[test]
	fn scan_range() {
		let mut scanner = Scanner::new(1,4);
		for _ in 0..2 {
			scanner.scan();
		}
		assert_eq!(scanner.scan(), 3);
	}

	#[test]
	fn scan_hit_top() {
		let mut scanner = Scanner::new(1,4);
		for _ in 0..5 {
			scanner.scan();
		}
		assert_eq!(scanner.scan(), 0);
	}

	#[test]
	fn scan_hit_top_bounce() {
		let mut scanner = Scanner::new(1,4);
		for _ in 0..6 {
			scanner.scan();
		}
		assert_eq!(scanner.scan(), 1);
	}
}