fn main() {
    let input: f64 = 289326.0;
	let ans = spiral_dist(input);
	println!("{}",ans);
}

fn spiral_dist(input: f64) -> f64 {
	let mut max_dist = input.sqrt().ceil();
	if max_dist % 2.0 == 1.0 {
		max_dist -=1.0;
	}
	((((max_dist + 1.0).powf(2.0) - input) % max_dist) - (max_dist/2.0)).abs() + max_dist/2.0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn spiral_dist_base() {
		assert_eq!(spiral_dist(1.0),0.0);
	}

	#[test]
	fn spiral_dist_small() {
		assert_eq!(spiral_dist(12.0),3.0);
	}

	#[test]
	fn spiral_dist_bigish() {
		assert_eq!(spiral_dist(1024.0),31.0);
	}

	#[test]
	fn spiral_dist_corner() {
		assert_eq!(spiral_dist(49.0),6.0);
	}
}
