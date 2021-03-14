#![allow(dead_code)]

use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

const TEST_DATA: [&str; 3] = [
	"1-3 a: abcde",
	"1-3 b: cdefg",
	"2-10 c: cccccccc"
];

const INPUT_PATH: &str = "02-password-philosophy/data/input.dat";

fn main() {
	let count = read_input(INPUT_PATH).unwrap()
		.iter()
		.filter(|phrase| valid_password(phrase).unwrap_or(false))
		.collect::<Vec<_>>()
		.len();

	println!("valid password: {}", count);
}

fn read_input(path: &str) -> Result<Vec<String>, &'static str> {
	let contents = fs::read_to_string(path)
		.map_err(|_| "File cannot be read")?;

	let lines: Vec<String> = contents.split("\n")
		.map(|line| line.to_string())
		.collect();

	println!("File line count: {}", lines.len());

	Ok(lines)
}

fn valid_password(phrase: &str) -> Result<bool, &'static str> {
	lazy_static! {
		static ref RE: Regex = Regex::new(r"^(\d+)-(\d+)\s+(.):\s+(\S+)$")
			.unwrap();
	}

	let captures = RE.captures(phrase).ok_or("unknown phrase for parsing")?;

	let min: u64 = captures.get(1)
		.ok_or("min value not found")?
		.as_str().parse()
		.map_err(|_| "unknown min value")?;

	let max: u64 = captures.get(2)
		.ok_or("max value not found")?
		.as_str().parse()
		.map_err(|_| "unknown max value")?;

	let check_char: char = captures.get(3)
		.ok_or("check char not found")?
		.as_str().parse()
		.map_err(|_| "unknown check char")?;

	let passphrase: &str = captures.get(4)
		.ok_or("pass phrase not found")?
		.as_str();

	let cnt: u64 = passphrase.chars()
		.filter(|c| *c == check_char)
		.collect::<Vec<_>>()
		.len() as u64;

	Ok(cnt >= min && cnt <= max)
}
