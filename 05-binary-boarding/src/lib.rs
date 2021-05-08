use std::fmt;
use std::collections::HashSet;

const ROWS: u64 = 128;
const COLS: u64 = 8;
const ALL_SEATS: u64 = ROWS * COLS;

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

pub fn find_empty_seats(seats: &[Seat]) -> HashSet<u64> {
	let mut seat_set: HashSet<u64> = HashSet::new();
	for n in 0..ALL_SEATS {
		seat_set.insert(n as u64);
	}

	for seat in seats.iter() {
		seat_set.remove(&seat.id());
	}

	// removing from the front, and removing from the end
	let mut low_end = 0;
	while seat_set.contains(&low_end) {
		seat_set.remove(&low_end);
		low_end += 1;
	}

	let mut high_end = ALL_SEATS - 1;
	while seat_set.contains(&high_end) {
		seat_set.remove(&high_end);
		high_end -= 1;
	}

	seat_set
}
