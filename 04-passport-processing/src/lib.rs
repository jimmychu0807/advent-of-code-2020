use lazy_static::lazy_static;
use regex::Regex;
// use std::error::Error;
use shared::{read_file};
use std::collections::HashMap;

#[cfg(test)]
mod tests;

const PASSPORT_REQ_KEYS: [&str; 7] = ["byr", "eyr", "iyr", "ecl", "hcl", "hgt", "pid"];

// -- ReadState enum --

#[derive(Clone, PartialEq, Debug)]
pub enum ReadState {
	Empty,
	Line,
}

impl Default for ReadState {
	fn default() -> Self {
		ReadState::Empty
	}
}

#[derive(Clone, PartialEq, Debug)]
pub enum PassportField {
	Num(u32),
	Str(String),
}

impl From<&str> for PassportField {
	fn from(val: &str) -> Self {
		Self::Str(val.to_string())
	}
}

impl From<u32> for PassportField {
	fn from(val: u32) -> Self {
		Self::Num(val)
	}
}

#[derive(Clone, PartialEq, Debug)]
pub enum PassportProcessError {
	UnknownKey(String),
	UnparsableU32(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum PassportInvalid {
	MissingField(String),
}

impl From<&str> for PassportInvalid {
	fn from(val: &str) -> Self {
		Self::MissingField(val.to_string())
	}
}

// -- Passport struct --
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Passport {
	pub fields: HashMap<String, PassportField>
}

impl Passport {
	pub fn new() -> Self {
		Passport::default()
	}

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
					let val = vec[1].parse::<u32>().map_err(|_| PassportProcessError::UnparsableU32(vec[1].to_string()))?;
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

	pub fn valid(&self) -> Result<(),Vec<PassportInvalid>> {
		let mut invalids = Vec::new();
		for key in PASSPORT_REQ_KEYS.iter() {
			if self.fields.get(*key).is_none() {
				invalids.push(PassportInvalid::from(*key));
			}
		}

		if !invalids.is_empty() { return Err(invalids) }
		Ok(())
	}
}

// -- other public functions --
pub fn read_from_file(input: &str) -> Result<Vec<Passport>, &'static str> {
	let lines = read_file(input)?;

	let mut state = ReadState::default();
	let mut passports: Vec<Passport> = Vec::new();
	let mut passport: Passport = Passport::new();

	for line in lines {
		let trimmed = line.trim();

		if trimmed.is_empty() {
			// current line is an empty line
			if let ReadState::Line = state {
				// prev line has content and reaching empty line now
				passports.push(passport.clone());
			}
			state = ReadState::Empty;
		} else {
			// current line is a non-empty line
			if let ReadState::Empty = state {
				// prev line is empty and reaching a line with content
				passport = Passport::new();
			}
			// process the line with content here
			passport = passport.process(trimmed).map_err(|_| "passport process error")?;
			state = ReadState::Line;
		}
	}

	Ok(passports)
}
