use passport_processing::{Passport, read_from_file};

const INPUT_PATH: &str = "04-passport-processing/data/input.dat";

fn main() -> Result<(), &'static str> {
	let _passports = read_from_file(INPUT_PATH)?;
	Ok(())
}
