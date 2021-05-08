use binary_boarding::Seat;

const INPUT_PATH: &str = "05-binary-boarding/data/input.dat";

fn main() -> Result<(), &'static str> {

	// let seat = Seat::new("FBFBBFFRLR");
	// println!("seat row: {}, col: {}, id: {}", seat.row(), seat.col(), seat.id());

	let lines = shared::read_file(INPUT_PATH)?;

	let max_seat = lines
		.iter()
		.filter(|line| !line.is_empty())
		.map(|line| Seat::new(line))
		.max_by(|x, y| x.id().cmp(&y.id()))
		.ok_or("No result returned")?;

	println!("Max seat: {:?}", max_seat);

	Ok(())
}
