use shared::{read_file_and_group, vec_string_to_str};
use custom_customs::{intersection_set, union_set};

const INPUT_PATH: &str = "06-custom-customs/data/input.dat";

fn main() -> Result<(), &'static str> {

	let groups = read_file_and_group(INPUT_PATH)?;

	// Part One
	let group_counts = groups.iter()
		.map(|grp| union_set(&vec_string_to_str(&grp[..])).len())
		.collect::<Vec<_>>();

	let sum = group_counts.iter()
		.fold(0, |acc, c| acc + c);

	println!("Part 1 solution: {:?}", sum);

	// Part Two
	let group_counts = groups.iter()
		.map(|grp| intersection_set(&vec_string_to_str(&grp[..])).len())
		.collect::<Vec<_>>();

	let sum = group_counts.iter()
		.fold(0, |acc, c| acc + c);

	println!("Part 2 solution: {:?}", sum);

	Ok(())
}
