use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

#[allow(dead_code)]
const TEST_DATA: [&str; 3] = [
	"1-3 a: abcde",
	"1-3 b: cdefg",
	"2-9 c: ccccccccc"
];

const INPUT_PATH: &str = "02-password-philosophy/data/input.dat";

struct PasswordSpec {
	first: u64,
	second: u64,
	test_char: char,
	passphrase: String
}

impl PasswordSpec {
	pub fn new(phrase: &str) -> Result<PasswordSpec, &'static str> {
		lazy_static! {
			static ref RE: Regex = Regex::new(r"^(\d+)-(\d+)\s+(.):\s+(\S+)$")
				.unwrap();
		}

		let captures = RE.captures(phrase).ok_or("unknown phrase for parsing")?;

		let first: u64 = captures.get(1)
			.ok_or("min value not found")?
			.as_str().parse()
			.map_err(|_| "unknown min value")?;

		let second: u64 = captures.get(2)
			.ok_or("max value not found")?
			.as_str().parse()
			.map_err(|_| "unknown max value")?;

		let test_char: char = captures.get(3)
			.ok_or("check char not found")?
			.as_str().parse()
			.map_err(|_| "unknown check char")?;

		let passphrase: String = captures.get(4)
			.ok_or("pass phrase not found")?
			.as_str().to_string();

		Ok(PasswordSpec { first, second, test_char, passphrase })
	}

	pub fn match_min_max(&self) -> bool {
		let cnt: u64 = self.passphrase.chars()
			.filter(|c| *c == self.test_char)
			.collect::<Vec<_>>()
			.len() as u64;

		cnt >= self.first && cnt <= self.second
	}

	pub fn match_one_pos(&self) -> bool {
		let chars: Vec<_> = self.passphrase.chars().collect();
		// Input is in 1-indexed
		let char_pos1 = chars[(self.first - 1) as usize];
		let char_pos2 = chars[(self.second - 1) as usize];
		let t = self.test_char;

		if char_pos1 == t && char_pos2 == t { return false }
		if char_pos1 != t && char_pos2 != t { return false }
		true
	}
}

fn main() {
	let lines = read_file(INPUT_PATH).unwrap();
	println!("File line count: {}", lines.len());

	let count_min_max = lines.iter()
		.filter_map(|l| PasswordSpec::new(l).ok())
		.filter(|ps| ps.match_min_max())
		.collect::<Vec<_>>()
		.len();

	println!("valid password for part 1: {}", count_min_max);

	let count_one_pos = lines.iter()
		.filter_map(|l| PasswordSpec::new(l).ok())
		.filter(|ps| ps.match_one_pos())
		.collect::<Vec<_>>()
		.len();

	println!("valid password for part 2: {}", count_one_pos);
}

fn read_file(path: &str) -> Result<Vec<String>, &'static str> {
	let contents = fs::read_to_string(path)
		.map_err(|_| "File cannot be read")?;

	let lines: Vec<String> = contents.split("\n")
		.map(|line| line.to_string())
		.collect();

	Ok(lines)
}
