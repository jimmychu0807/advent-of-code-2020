#![allow(dead_code)]

use shared::{read_file_and_group, vec_string_to_str};
use handy_haversacks::{Haversacks};

const TEST_PATH: &str = "07-handy-haversacks/data/test.dat";

const INPUT_PATH: &str = "07-handy-haversacks/data/input.dat";

fn main() -> Result<(), &'static str> {

	let groups = read_file_and_group(INPUT_PATH)?;
	let groups: Vec<_> = groups.into_iter().flatten().collect();

	let haversacks = Haversacks::setup(vec_string_to_str(&groups));
	let num = haversacks.find_contained_qty(&"shiny gold");

	println!("{} bag colors can eventually contain at least one shiny gold bag.", num);

	Ok(())
}
