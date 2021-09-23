use shared::{read_file_and_group};
use handheld_halting::{Compiler};

#[allow(dead_code)]
const TEST_PATH: &str = "08-handheld-halting/data/test.dat";

#[allow(dead_code)]
const INPUT_PATH: &str = "08-handheld-halting/data/input.dat";

fn main() -> Result<(), String> {
	let ins = read_file_and_group(TEST_PATH)?;
	let ins: Vec<_> = ins.into_iter().flatten().collect();

	// Part 1
	let compiler = Compiler::new(ins, false);
	let result = compiler.execute()?;
	println!("res: {:?}", result);

	// Part 2
	let all_sets = ins.iter().enumerate()
		.filter(|(i, cmd)| cmd.starts_with("jmp") || cmd.starts_with("nop"))
		.map(|(i, cmd)| {
			let mut new_set = ins.clone();
			new_set[i] = if new_set[i].starts_with("jmp") {
				new_set[i].replace("jmp", "nop")
			} else {
				new_set[i].replace("nop", "jmp")
			};
			new_set
		});

	println!("all_sets: {:?}", all_sets);

	Ok(())
}
