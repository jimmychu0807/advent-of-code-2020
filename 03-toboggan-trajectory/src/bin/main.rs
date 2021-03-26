use std::convert::TryFrom;

use toboggan_trajectory::Slope;
use shared::{read_file, vec_string_to_str};

const INPUT_PATH: &str = "03-toboggan-trajectory/data/input01.dat";

const PART_A: (u64, u64) = (3, 1);

fn main() -> Result<(), &'static str> {
	// Part 1, sliding sideway of 3
	let lines = read_file(INPUT_PATH).unwrap();
	let lines_str: Vec<&str> = vec_string_to_str(&lines);
	let slope = Slope::try_from(lines_str)?;
	let hits = slope.slide(PART_A.0, PART_A.1);

	println!("# of hit: {}", hits);

	Ok(())
}
