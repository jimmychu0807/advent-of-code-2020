use binary_boarding::{Seat, find_empty_seats};

const INPUT_PATH: &str = "05-binary-boarding/data/input.dat";

fn main() -> Result<(), &'static str> {
	let lines = shared::read_file(INPUT_PATH)?;

	// Part A
	let seats = lines
		.iter()
		.filter(|line| !line.is_empty())
		.map(|line| Seat::new(line))
		.collect::<Vec<_>>();

	let max_seat = seats
		.iter()
		.max_by(|x, y| x.id().cmp(&y.id()))
		.ok_or("No result returned")?;

	println!("Max seat: {:?}", max_seat);

	// Part B
	let empty_seats = find_empty_seats(&seats);
	println!("Empty seat set: {:?}", empty_seats);

	Ok(())
}
