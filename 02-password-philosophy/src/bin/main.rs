use password_philosophy::PasswordSpec;
use shared::{read_file};

const INPUT_PATH: &str = "02-password-philosophy/data/input.dat";

fn main() {
	let lines = read_file(INPUT_PATH).unwrap();
	println!("File line count: {}", lines.len());

	let count_min_max = lines.iter()
		.filter_map(|l| PasswordSpec::new(l).ok())
		.filter(|ps| ps.match_min_max())
		.count();

	println!("valid password for part 1: {}", count_min_max);

	let count_one_pos = lines.iter()
		.filter_map(|l| PasswordSpec::new(l).ok())
		.filter(|ps| ps.match_one_pos())
		.count();

	println!("valid password for part 2: {}", count_one_pos);
}
