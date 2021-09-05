use shared::{read_file_and_group, vec_string_to_str};
use handy_haversacks::{build_rules};

const TEST_PATH: &str = "07-handy-haversacks/data/test.dat";

const INPUT_PATH: &str = "07-handy-haversacks/data/input.dat";

fn main() -> Result<(), &'static str> {

	let groups = read_file_and_group(TEST_PATH)?;
	let groups: Vec<_> = groups.into_iter().flatten().collect();

	println!("{:?}", groups);

	let haversacks = build_rules(vec_string_to_str(&groups));
	let num = haversacks.find_outer_qty(&"shiny gold");

	println!("{:?}", num);

	Ok(())
}
