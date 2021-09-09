use shared::{read_file_and_group, vec_string_to_str};
use handy_haversacks::{Haversacks};

#[allow(dead_code)]
const TEST1_PATH: &str = "07-handy-haversacks/data/test1.dat";

#[allow(dead_code)]
const TEST2_PATH: &str = "07-handy-haversacks/data/test2.dat";

#[allow(dead_code)]
const INPUT_PATH: &str = "07-handy-haversacks/data/input.dat";

fn main() -> Result<(), &'static str> {
	let groups = read_file_and_group(INPUT_PATH)?;
	let groups: Vec<_> = groups.into_iter().flatten().collect();

	let haversacks = Haversacks::setup(vec_string_to_str(&groups));
	let qty = haversacks.contained_qty(&"shiny gold");
	println!("{} bag colors can eventually contain at least one shiny gold bag.", qty);

	let sum = haversacks.contained_sum(&"shiny gold");
	println!("{} bags are required inside your single shiny gold bag.", sum);

	Ok(())
}
