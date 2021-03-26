use std::convert::TryFrom;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Slope {
	landscape: Vec<Vec<u8>>
}

impl TryFrom<Vec<&str>> for Slope {
	type Error = &'static str;

	fn try_from (input: Vec<&str>) -> Result<Self, Self::Error> {
		let landscape = input.into_iter().filter_map(|line| {
			if line.is_empty() { return None }
			Some(
				line.chars().map(|c| match c {
					'.' => Ok(0),
					'#' => Ok(1),
					_ => Err("Unknown character in input")
				}).collect::<Result<Vec<_>, _>>()
			)
		}).collect::<Result<Vec<_>, _>>()?;
		// This above two `collect`s used a rust trick. An iterator of Result<_,_> when collected, can
		//   return to a Result of vector.
		//   Ref: https://stackoverflow.com/questions/26368288/how-do-i-stop-iteration-and-return-an-error-when-iteratormap-returns-a-result

		Ok(Slope { landscape })
	}
}

impl Slope {
	pub fn slide(&self, mv_right: u64, mv_down: u64) -> u64 {
		let col_hgt: u64 = self.landscape.len() as u64;
		let row_len: u64 = self.landscape[0].len() as u64;

		let mut x: usize = 0;
		let mut y: usize = 0;
		let mut hits: u64 = 0;

		while y < (col_hgt as usize) {
			hits += self.landscape[y][x] as u64;

			// processing the x,y indices
			y += mv_down as usize;
			x += mv_right as usize;
			if x >= (row_len as usize) { x -= row_len as usize }
		}

		hits
	}
}
