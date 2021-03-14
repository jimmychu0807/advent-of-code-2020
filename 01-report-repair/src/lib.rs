pub struct ReportRepair {
	numbers: Vec<u64>
}

impl ReportRepair {

	pub fn new(numbers: Vec<u64>) -> ReportRepair {
		ReportRepair { numbers }
	}

	pub fn sum_two_to_target(&self, target: u64) -> Option<(u64, u64)> {

		let mut ans: Option<(u64, u64)> = None;

		self.numbers.iter().enumerate().for_each(|(i1, v1)| {
			self.numbers.iter().enumerate().for_each(|(i2, v2)| {
				// check if answer have already been found
				if ans.is_some() { return }
				// we exclude indexing the same value twice
				if i1 == i2 { return }
				// this is not what we want
				if v1 + v2 != target { return }

				ans = Some((*v1, *v2));
			});
		});
		ans
	}

	pub fn sum_three_to_target(&self, target: u64) -> Option<(u64, u64, u64)> {
		let mut ans: Option<(u64, u64, u64)> = None;

		self.numbers.iter().enumerate().for_each(|(i1, v1)| {
			self.numbers.iter().enumerate().for_each(|(i2, v2)| {
				self.numbers.iter().enumerate().for_each(|(i3, v3)| {
					// check if answer have already been found
					if ans.is_some() { return }
					// we exclude indexing the same value twice
					if i1 == i2 || i2 == i3 { return }
					// this is not what we want
					if v1 + v2 + v3 != target { return }

					ans = Some((*v1, *v2, *v3));
				});
			});
		});

		ans
	}

}
