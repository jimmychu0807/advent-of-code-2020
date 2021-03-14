use regex::Regex;
use lazy_static::lazy_static;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct PasswordSpec {
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
			.count() as u64;

		cnt >= self.first && cnt <= self.second
	}

	pub fn match_one_pos(&self) -> bool {
		let chars: Vec<_> = self.passphrase.chars().collect();
		// Input is in 1-indexed
		let char_pos1 = chars.get((self.first - 1) as usize);
		let char_pos2 = chars.get((self.second - 1) as usize);
		let t = self.test_char;

		match (char_pos1, char_pos2) {
			(Some(char_pos1), Some(char_pos2)) => {
				if *char_pos1 == t && *char_pos2 == t { return false }
				if *char_pos1 != t && *char_pos2 != t { return false }
				true
			},
			(Some(char_pos1), None) => *char_pos1 == t,
			(None, Some(char_pos2)) => *char_pos2 == t,
			(None, None) => false,
		}
	}
}
