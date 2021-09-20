use shared::{read_file_and_group};
use handheld_halting::{Compiler};

#[allow(dead_code)]
const TEST_PATH: &str = "08-handheld-halting/data/test.dat";

#[allow(dead_code)]
const INPUT_PATH: &str = "08-handheld-halting/data/input.dat";

fn main() -> Result<(), String> {
	let ins = read_file_and_group(INPUT_PATH)?;
	let ins: Vec<_> = ins.into_iter().flatten().collect();
	let compiler = Compiler::new(ins, true);
	let result = compiler.execute()?;

	println!("res: {:?}", result);

	Ok(())
}
