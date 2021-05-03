use passport_processing::read_from_file;

const INPUT_PATH: &str = "04-passport-processing/data/input.dat";

fn main() -> Result<(), &'static str> {
	let passports = read_from_file(INPUT_PATH)?;
	let valid_passport_cnt = passports.iter().filter(|p| p.valid().is_ok()).count();
	println!("Part One: valid entries: {}", valid_passport_cnt);
	Ok(())
}
