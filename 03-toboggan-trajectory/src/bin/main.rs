use std::convert::TryFrom;

use toboggan_trajectory::Slope;
use shared::{read_file, vec_string_to_str};

const INPUT_PATH: &str = "03-toboggan-trajectory/data/input01.dat";

const PART_1: (u64, u64) = (3, 1);
const PART_2: [(u64, u64); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn main() -> Result<(), &'static str> {
	let lines = read_file(INPUT_PATH)?;
	let lines_str: Vec<&str> = vec_string_to_str(&lines);
	let slope = Slope::try_from(lines_str)?;

	// Part One, sliding sideway of 3
	let hits = slope.slide(PART_1.0, PART_1.1);
	println!("Part One: # of hit = {}", hits);

	// Part Two
	let input_slides = PART_2.to_vec();
	let res = input_slides.iter().map(|(mv_right, mv_down)|
		slope.slide(*mv_right, *mv_down)
	).collect::<Vec<_>>();

	for (ind, item) in input_slides.iter().enumerate() {
		println!("({}, {}) hits: {}", item.0, item.1, res[ind]);
	}

	let product = res.iter().fold(1, |hits, m| m * hits);
	println!("Part Two: Product = {}", product);

	Ok(())
}
