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
	OutOfRange(String, String),
	InvalidFormat(String, String),
}

impl From<&str> for PassportInvalid {
	fn from(val: &str) -> Self {
		Self::MissingField(val.to_string())
	}
}

impl PassportInvalid {
	pub fn out_of_range(key: &str, val: &str) -> Self {
		Self::OutOfRange(key.to_string(), val.to_string())
	}

	pub fn invalid_format(key: &str, val: &str) -> Self {
		Self::InvalidFormat(key.to_string(), val.to_string())
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

	pub fn validate_simplified(&self) -> Result<(),Vec<PassportInvalid>> {
		let mut invalids = Vec::new();

		for key in PASSPORT_REQ_KEYS.iter() {
			if self.fields.get(*key).is_none() {
				invalids.push(PassportInvalid::from(*key));
			}
		}

		if !invalids.is_empty() { return Err(invalids) }
		Ok(())
	}

	pub fn validate(&self) -> Result<(),Vec<PassportInvalid>> {
		const ECL_VALS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
		lazy_static! {
			// This unwrap() will not cause error.
			static ref HGT_REGEX: Regex = Regex::new(r"^(\d+)(\w+)$").unwrap();
			static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
			static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
		}

		let mut invalids = Vec::new();

		for &key in PASSPORT_REQ_KEYS.iter() {
			match self.fields.get(key) {
				Some(&PassportField::Num(val)) if key == "byr" && !(val >= 1920 && val <= 2002) => {
					invalids.push(PassportInvalid::out_of_range(key, &val.to_string()));
				},

				Some(&PassportField::Num(val)) if key == "iyr" && !(val >= 2010 && val <= 2020) => {
					invalids.push(PassportInvalid::out_of_range(key, &val.to_string()));
				},

				Some(&PassportField::Num(val)) if key == "eyr" && !(val >= 2020 && val <= 2030) => {
					invalids.push(PassportInvalid::out_of_range(key, &val.to_string()));
				},

				Some(&PassportField::Str(ref val)) if key == "hgt" => {
					if let Some(captures) = HGT_REGEX.captures(val) {
						if let (Some(measure), Some(unit)) = (captures.get(1), captures.get(2)) {
							let measure_str = measure.as_str();
							let unit_str = unit.as_str();

							if unit_str != "cm" && unit_str != "in" {
								invalids.push(PassportInvalid::invalid_format(key, val));
							}

							if let Ok(hgt_val) = measure_str.parse::<u32>() {
								if (unit_str == "cm" && !(hgt_val >= 150 && hgt_val <= 193)) ||
									(unit_str == "in" && !(hgt_val >= 59 && hgt_val <= 76)) {
									invalids.push(PassportInvalid::out_of_range(key, val));
								}
							} else {
								invalids.push(PassportInvalid::invalid_format(key, val));
							}
						} else {
							invalids.push(PassportInvalid::invalid_format(key, val));
						}
					} else {
						// Capture nothing here. Regard as invalid format.
						invalids.push(PassportInvalid::invalid_format(key, val));
					}
				},

				Some(&PassportField::Str(ref val)) if key == "hcl" && !HCL_REGEX.is_match(val) => {
					invalids.push(PassportInvalid::invalid_format(key, val));
				},

				Some(&PassportField::Str(ref val)) if key == "ecl" && !ECL_VALS.contains(&val.as_ref()) => {
					invalids.push(PassportInvalid::invalid_format(key, val));
				},

				Some(&PassportField::Str(ref val)) if key == "pid" && !PID_REGEX.is_match(val) => {
					invalids.push(PassportInvalid::invalid_format(key, val));
				},

				Some(_) => {}, // this line is needed so those pass the above will become accepted here

				None => { invalids.push(PassportInvalid::from(key)) },
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
