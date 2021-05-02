use lazy_static::lazy_static;
use regex::Regex;
// use std::error::Error;
use shared::{read_file};
use std::collections::HashMap;

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

#[derive(Clone, PartialEq, Debug)]
pub enum PassportField {
	Num(u32),
	Str(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum PassportProcessError {
	UnknownKey(String),
	UnparsableU32(String),
}

pub const PASSPORT_REQ_KEYS: [&str; 7] = ["byr", "eyr", "iyr", "ecl", "hcl", "hgt", "pid"];

// -- Passport struct --
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Passport {
	pub fields: HashMap<String, PassportField>
}

impl Passport {
	pub fn process(mut self, line: &str) -> Result<Self, PassportProcessError> {
		lazy_static! {
			// This unwrap() will not cause error.
			static ref SPACES: Regex = Regex::new(r"\s+").unwrap();
		}

		let key_vals: Vec<&str> = SPACES.split(line).collect();
		for key_val in key_vals {
			let vec: Vec<&str> = key_val.split(':').collect();
			match vec[0] {
				"byr" | "eyr" | "iyr" => {
					let val = vec[1].parse::<u32>().or_else(|_|
						Err(PassportProcessError::UnparsableU32(vec[1].to_string())))?;
					self.fields.insert(vec[0].to_string(), PassportField::Num(val));
				},
				"ecl" | "hcl" | "hgt" | "pid" | "cid" => {
					self.fields.insert(vec[0].to_string(), PassportField::Str(vec[1].to_string()));
				}
				_ => { return Err(PassportProcessError::UnknownKey(vec[0].to_string())) }
			};
		}

		Ok(self)
	}

	pub fn valid(&self) -> bool {
		for key in PASSPORT_REQ_KEYS.iter() {
			if let None = self.fields.get(*key) { return false }
		}
		true
	}
}

// -- other public functions --
pub fn read_from_file(input: &str) -> Result<Vec<Passport>, &'static str> {
	let lines = read_file(input)?;

	let mut state = ReadState::default();
	let mut passports: Vec<Passport> = Vec::new();
	let mut passport: Passport = Passport::default();

	for line in lines {
		let trimmed = line.trim();

		if trimmed.len() == 0 {
			// current line is an empty line
			if let ReadState::LINE = state {
				// prev line has content and reaching empty line now
				passports.push(passport.clone());
			}
			state = ReadState::EMPTY;
		} else {
			// current line is a non-empty line
			if let ReadState::EMPTY = state {
				// prev line is empty and reaching a line with content
				passport = Passport::default();
			}
			// process the line with content here
			passport = passport.process(trimmed).or_else(|_| Err("passport process error"))?;
			state = ReadState::LINE;
		}
	}

	Ok(passports)
}
