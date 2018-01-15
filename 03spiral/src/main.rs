use std::ops::Index;
use std::ops::IndexMut;

const SIZE : usize = 11;

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

#[derive(Clone)]
struct Point {
	x: usize,
	y: usize,
}

impl Point {
	fn translate(&self, (x_delta, y_delta): (i32, i32)) -> Point {
		Point{x:(self.x as i32 + x_delta) as usize, y:(self.y as i32 + y_delta) as usize}
	}
}

impl Index<Point> for [[u32;SIZE];SIZE] {
	type Output = u32;
	fn index(&self, point: Point) -> &u32 {
		&self[point.x][point.y]
	}
}

impl IndexMut<Point> for [[u32;SIZE];SIZE] {
	fn index_mut(&mut self, point: Point) -> &mut u32 {
		&mut self[point.x][point.y]
	}
}

fn sum_spiral(limit: u32) -> u32 {

	let mut vec = [[0;SIZE];SIZE];
	let mut num = 1;
	let mut loc = Point {x: SIZE / 2, y: SIZE / 2};
	vec[loc.clone()] = num;

	let movement = [(0,1),(-1,0),(0,-1),(1,0)];
	let mut movement_index = 0;
	let mut movement_done = false;
	let mut movement_iterations = 1;

	loop {

		for _ in 0..movement_iterations {
			loc = loc.translate(movement[movement_index]);
			num = find_surrounding_sum(&loc, &vec);
			vec[loc.clone()] = num;
			if num > limit {
				print_2d_array(&vec);
				return num;
			}
		}
		movement_index = (movement_index + 1) % movement.len();
		if movement_done {
			movement_done = !movement_done;
			movement_iterations+=1;
		} else {
			movement_done = !movement_done;
		}
	}
}

// Gotta have a size big enough to keep an outer border
fn find_surrounding_sum(loc: &Point, vec : &[[u32; SIZE];SIZE]) -> u32 {
	let rt = vec[loc.translate((0,1))];
	let lt = vec[loc.translate((0,-1))];
	let up = vec[loc.translate((-1,0))];
	let dn = vec[loc.translate((1,0))];
	let rt_up = vec[loc.translate((-1,1))];
	let rt_dn = vec[loc.translate((1,1))];
	let lt_up = vec[loc.translate((-1,-1))];
	let lt_dn = vec[loc.translate((1,-1))];
	rt + lt + up + dn + rt_up + rt_dn + lt_up + lt_dn
}

fn print_2d_array(array: &[[u32; SIZE];SIZE]) {
    for arr in array.into_iter() {
		for elem in arr.iter() {
			print!("{:6}, ", elem);
		}
		println!("");
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
