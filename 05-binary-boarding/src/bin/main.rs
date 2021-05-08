use binary_boarding::Seat;

const INPUT_PATH: &str = "05-binary-boarding/data/input.dat";

fn main() -> Result<(), &'static str> {

	let seat = Seat::new("FFFFBBBRLR");
	println!("seat row: {}, col: {}, id: {}", seat.row(), seat.col(), seat.id());

	let lines = shared::read_file(INPUT_PATH)?;
	let max = lines.iter().map(|line| Seat::new(line).id()).max();
	println!("Max seat ID: {:?}", max);

	Ok(())
}
