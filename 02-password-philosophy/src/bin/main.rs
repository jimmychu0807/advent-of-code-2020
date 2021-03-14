use std::fs;
use password_philosophy::PasswordSpec;

const INPUT_PATH: &str = "02-password-philosophy/data/input.dat";

fn read_file(path: &str) -> Result<Vec<String>, &'static str> {
	let contents = fs::read_to_string(path)
		.map_err(|_| "File cannot be read")?;

	let lines: Vec<String> = contents.split('\n')
		.map(|line| line.to_string())
		.collect();

	Ok(lines)
}

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
