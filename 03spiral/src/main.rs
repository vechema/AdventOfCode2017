const size : usize = 11;

fn main() {
    let input: f64 = 289326.0;
	let ans = spiral_dist(input);
	println!("{}",ans);

	let ans2 = sum_spiral(input as u32);
	println!("{}",ans2);
}

fn spiral_dist(input: f64) -> f64 {
	let mut max_dist = input.sqrt().ceil();
	if max_dist % 2.0 == 1.0 {
		max_dist -=1.0;
	}
	((((max_dist + 1.0).powf(2.0) - input) % max_dist) - (max_dist/2.0)).abs() + max_dist/2.0
}

fn sum_spiral(limit: u32) -> u32 {

	let mut vec = [[0;size];size];
	let mut num = 1;
	let mut x = size / 2;
	let mut y = size / 2;
	vec[x][y] = num;

	let mut result = std::u32::MAX;

	let movement = [(0,1),(-1,0),(0,-1),(1,0)];
	let mut movement_index = 0;
	let mut movement_done = false;
	let mut movement_iterations = 1;

	while num <= limit{

		for iter in 0..movement_iterations {
			x = move_x_coordinate(x,movement[movement_index]);
			y = move_y_coordinate(y,movement[movement_index]);
			num = find_surrounding_sum(x, y, &vec);
			if num > limit {
				result = num;
				break;
			}
			vec[x][y] = num;
		}
		movement_index = (movement_index + 1) % movement.len();
		if movement_done {
			movement_done = !movement_done;
			movement_iterations+=1;
		} else {
			movement_done = !movement_done;
		}
	}
	print_2d_array(&vec);

	result
}

fn move_x_coordinate(x: usize, change: (i32, i32)) -> usize {
	let (x_delta, y_delta) = change;
	(x as i32+ x_delta) as usize
}

fn move_y_coordinate(y: usize, change: (i32, i32)) -> usize {
	let (x_delta, y_delta) = change;
	(y as i32 + y_delta) as usize
}

// Gotta have a size big enough to keep an outer border
fn find_surrounding_sum(x : usize, y : usize, vec : &[[u32; size];size]) -> u32 {
	let rt = vec[x][y + 1];
	let lt = vec[x][y - 1];
	let up = vec[x - 1][y];
	let dn = vec[x + 1][y];
	let rt_up = vec[x - 1][y + 1];
	let rt_dn = vec[x + 1][y + 1];
	let lt_up = vec[x - 1][y - 1];
	let lt_dn = vec[x + 1][y - 1];
	vec[x][y] + rt + lt + up + dn + rt_up + rt_dn + lt_up + lt_dn
}

fn print_2d_array(array: &[[u32; size];size]) {
    for arr in array.into_iter() {
		println!("{:?}", arr);
	}
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
