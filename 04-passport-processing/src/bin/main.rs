use passport_processing::read_from_file;

const INPUT_PATH: &str = "04-passport-processing/data/input.dat";

fn main() -> Result<(), &'static str> {
	let (passports, errors) = read_from_file(INPUT_PATH)?;
	println!("Part One: valid entries: {}", passports.len());
	println!("Part One: invalid entries: {}", errors.len());
	Ok(())
}
