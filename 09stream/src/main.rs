extern crate utilities;

fn main() {
    let input = utilities::read_file("input.txt");

	let (total_score, garbage_char_count) = calculate_total_score(&input);
	println!("{}", total_score);
	println!("{}", garbage_char_count);
}

fn calculate_total_score(groups: &String) -> (u32, u32) {
	let mut level = 0;
	let mut score = 0;
	let mut in_garbage = false;
	let mut cancel = false;
	let mut garbage_char_count = 0;

	for char in groups.chars() {
		if in_garbage && !cancel && char != '!' && char != '>' {
			garbage_char_count+=1;
		}

		if cancel {
			cancel = false;
		} else if char == '!' {
			cancel = true;
		} else if char == '{'  && !in_garbage {
			level+=1;
			score+=level;
		} else if char == '}' && !in_garbage {
			level-=1;
		} else if char == '<' {
			in_garbage = true;
		} else if char == '>' {
			in_garbage = false;
		}
	}

	(score, garbage_char_count)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn score_base() {
		let groups = String::from("{}");
		assert_eq!(calculate_total_score(&groups), 1);
	}

	#[test]
	fn score_all_nested() {
		let groups = String::from("{{{}}}");
		assert_eq!(calculate_total_score(&groups), 6);
	}

	#[test]
	fn score_list() {
		let groups = String::from("{{},{}}");
		assert_eq!(calculate_total_score(&groups), 5);
	}

	#[test]
	fn score_list_and_nested() {
		let groups = String::from("{{{},{},{{}}}}");
		assert_eq!(calculate_total_score(&groups), 16);
	}

	#[test]
	fn score_garbage_base() {
		let groups = String::from("{<a>,<a>,<a>,<a>}");
		assert_eq!(calculate_total_score(&groups), 1);
	}

	#[test]
	fn score_garbage_base_group_in() {
		let groups = String::from("{<{}>,<a>,<a>,<a>}");
		assert_eq!(calculate_total_score(&groups), 1);
	}

	#[test]
	fn score_garbage_cancel() {
		let groups = String::from("{{<!!>},{<!!>},{<!!>},{<!!>}}");
		assert_eq!(calculate_total_score(&groups), 9);
	}

	#[test]
	fn score_garbage_cancel_mix() {
		let groups = String::from("{{<a!>},{<a!>},{<a!>},{<ab>}}");
		assert_eq!(calculate_total_score(&groups), 3);
	}
}
