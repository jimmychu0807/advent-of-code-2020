#[derive(Debug)]
pub struct Solver {
	input: Vec<u64>
}

impl Solver {
	pub fn new(input: &Vec<u64>) -> Self {
		Solver {
			input: input.clone()
		}
	}

	pub fn find_error(&self, preamble: u64) -> u64 {
		0
	}
}
