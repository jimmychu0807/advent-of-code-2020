use passport_processing::read_from_file;

const INPUT_PATH: &str = "04-passport-processing/data/input.dat";

fn main() -> Result<(), &'static str> {
	let passports = read_from_file(INPUT_PATH)?;
	let valid_passport_cnt1 = passports.iter().filter(|p| p.validate_simplified().is_ok()).count();
	println!("Part One: valid entries: {}", valid_passport_cnt1);

	let valid_passport_cnt2 = passports.iter().filter(|p| p.validate().is_ok()).count();
	println!("Part Two: valid entries: {}", valid_passport_cnt2);

	Ok(())
}
