use shared::{read_file_and_group};
use encoding_error::{Solver};

#[allow(dead_code)]
const TEST_PATH: &str = "09-encoding-error/data/test.dat";

#[allow(dead_code)]
const INPUT_PATH: &str = "09-encoding-error/data/input.dat";

fn main() -> Result<(), String> {
	let input = read_file_and_group(TEST_PATH)?;
	let input: Vec<u64> = input.into_iter()
		.flatten()
		.map(|v| v.parse::<u64>().expect("Input file should be a value"))
		.collect();

	let solver = Solver::new(&input);
	let result = solver.find_error(5);

	Ok(())
}
