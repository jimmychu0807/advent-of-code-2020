use lazy_static::lazy_static;
use regex::Regex;
// use std::error::Error;
use shared::{read_file};

#[cfg(test)]
mod tests;

// -- ReadState enum --

#[derive(Clone, PartialEq, Debug)]
pub enum ReadState {
	EMPTY,
	LINE,
}

impl Default for ReadState {
	fn default() -> Self {
		ReadState::EMPTY
	}
}

// -- Passport struct --

#[derive(Clone, PartialEq, Debug)]
pub struct Passport {
	byr: u32,
	eyr: u32,
	iyr: u32,
	ecl: String,
	hcl: String,
	hgt: String,
	pid: String,
	cid: Option<String>,
}

impl Passport {
	pub fn builder() -> PassportBuilder {
		PassportBuilder::default()
	}
}

// -- PassportBuilderError struct --

#[derive(Debug)]
pub struct PassportBuilderError {
	err: String
}

impl PassportBuilderError {
	pub fn new(err: &str) -> Self {
		PassportBuilderError { err: err.to_string() }
	}
}

// -- PassportBuilder struct --

#[derive(Default, Clone, PartialEq)]
pub struct PassportBuilder {
	byr: Option<u32>,
	eyr: Option<u32>,
	iyr: Option<u32>,
	ecl: Option<String>,
	hcl: Option<String>,
	hgt: Option<String>,
	pid: Option<String>,
	cid: Option<String>,
}

impl PassportBuilder {
	pub fn process(mut self, line: &str) -> Result<Self, &'static str> {
		lazy_static! {
			// This unwrap() will not cause error.
			static ref SPACES: Regex = Regex::new(r"\s+").unwrap();
		}
		let key_vals: Vec<&str> = SPACES.split(line).collect();
		for key_val in key_vals {
			let vec: Vec<&str> = key_val.split(':').collect();
			match vec[0] {
				"byr" => self.byr = Some(vec[1].parse::<u32>()
					.or_else(|_| Err("Parsing byr to u32 failed"))?),
				"eyr" => self.eyr = Some(vec[1].parse::<u32>()
					.or_else(|_| Err("Parsing eyr to u32 failed"))?),
				"iyr" => self.iyr = Some(vec[1].parse::<u32>()
					.or_else(|_| Err("Parsing iyr to u32 failed"))?),
				"ecl" => self.ecl = Some(vec[1].to_string()),
				"hcl" => self.hcl = Some(vec[1].to_string()),
				"hgt" => self.hgt = Some(vec[1].to_string()),
				"pid" => self.pid = Some(vec[1].to_string()),
				"cid" => self.cid = Some(vec[1].to_string()),
				_ => { return Err("Unknown key for PassportBuilder") }
			};
		}

		Ok(self)
	}

	pub fn build(self) -> Result<Passport, &'static str> {
		Ok(Passport {
			byr: self.byr.ok_or("byr missing")?,
			eyr: self.eyr.ok_or("eyr missing")?,
			iyr: self.iyr.ok_or("iyr missing")?,
			ecl: self.ecl.ok_or("ecl missing")?.to_string(),
			hcl: self.hcl.ok_or("hcl missing")?.to_string(),
			hgt: self.hgt.ok_or("hgt missing")?.to_string(),
			pid: self.pid.ok_or("pid missing")?.to_string(),
			cid: self.cid.map(|s| s.to_string()),
		})
	}
}

// -- other public functions --

pub fn read_from_file(input: &str) -> Result<(Vec<Passport>, Vec<PassportBuilderError>), &'static str> {
	let lines = read_file(input)?;

	let mut state = ReadState::default();
	let mut passports = Vec::new();
	let mut errors = Vec::new();
	let mut builder: Option<PassportBuilder> = None;

	for line in lines {
		let trimmed = line.trim();

		if trimmed.len() == 0 {
			if let ReadState::LINE = state {
				// prev line has content and reaching empty line now
				if let Some(builder) = builder.take() {
					match builder.build() {
						Ok(passport) => { passports.push(passport) }
						Err(err) => { errors.push(PassportBuilderError::new(err)) }
					}
				}
			}
			state = ReadState::EMPTY;
		} else {
			if let ReadState::EMPTY = state {
				// prev line is empty and reaching a line with content
				builder = Some(Passport::builder());
			}
			// process the line with content here
			builder = Some(builder.ok_or("Passport builder is double-spent")?.process(trimmed)?);
			state = ReadState::LINE;
		}
	}

	Ok((passports, errors))
}
