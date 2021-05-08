const ROW_SEATS: u64 = 8;

pub struct Seat(String);

impl Seat {
	pub fn new(input: &str) -> Self {
		Seat(input.to_string())
	}

	pub fn row(&self) -> u64 {
		1
	}

	pub fn col(&self) -> u64 {
		1
	}

	pub fn id(&self) -> u64 {
		self.row() * ROW_SEATS + self.col()
	}
}
