use std::fmt;

const ROWS: u64 = 128;
const COLS: u64 = 8;

pub struct Seat(String);

impl fmt::Debug for Seat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Seat")
			.field("str", &self.0)
			.field("row", &self.row())
			.field("col", &self.col())
			.field("id", &self.id())
			.finish()
	}
}

impl Seat {
	pub fn new(input: &str) -> Self {
		Seat(input.to_string())
	}

	pub fn row(&self) -> u64 {
		let row_str = &self.0[0..=6];
		let mut lower = 0;
		let mut upper = ROWS - 1;
		for row_char in row_str.chars() {
			if row_char == 'B' {
				lower = (lower + upper + 1) / 2;
			} else {
				upper = (lower + upper + 1) / 2 - 1;
			}
		}
		assert!(lower == upper, "row: {}, lower: {}, upper: {} are not equal!", row_str, lower, upper);
		lower
	}

	pub fn col(&self) -> u64 {
		let col_str = &self.0[7..=9];
		let mut lower = 0;
		let mut upper = COLS - 1;
		for col_char in col_str.chars() {
			if col_char == 'R' {
				lower = (lower + upper + 1) / 2;
			} else {
				upper = (lower + upper + 1) / 2 - 1;
			}
		}
		assert!(lower == upper, "col: {}, lower: {}, upper: {} are not equal!", col_str, lower, upper);
		lower
	}

	pub fn id(&self) -> u64 {
		self.row() * COLS + self.col()
	}
}
