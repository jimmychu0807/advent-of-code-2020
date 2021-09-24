use shared::{read_file_and_group};
use handheld_halting::{Compiler, ExecutedResult, ExitStatus};

#[allow(dead_code)]
const TEST_PATH: &str = "08-handheld-halting/data/test.dat";

#[allow(dead_code)]
const INPUT_PATH: &str = "08-handheld-halting/data/input.dat";

fn main() -> Result<(), String> {
	let ins = read_file_and_group(INPUT_PATH)?;
	let ins: Vec<_> = ins.into_iter().flatten().collect();

	// Part 1
	let compiler = Compiler::new(&ins, false);
	let result = compiler.execute()?;
	println!("Result: {:?}", result);

	// Part 2
	let all_sets: Vec<_> = ins.iter().enumerate()
		.filter(|(_i, cmd)| cmd.starts_with("jmp") || cmd.starts_with("nop"))
		.map(|(i, cmd)| {
			let mut new_set = ins.clone();

			let new_ins = if cmd.starts_with("jmp") {
				cmd.replace("jmp", "nop")
			} else if cmd.starts_with("nop") {
				cmd.replace("nop", "jmp")
			} else {
				cmd.to_string()
			};
			new_set[i] = new_ins;
			new_set
		})
		.collect();

	let mut found_ins: Option<&Vec<_>> = None;
	let mut found_result: Option<ExecutedResult> = None;
	for ins in all_sets.iter() {
		let compiler = Compiler::new(&ins, false);
		let result = compiler.execute()?;
		if let ExitStatus::Okay = result.status {
			found_ins = Some(&ins);
			found_result = Some(result.clone());
			break;
		}
	}

	match (found_result, found_ins) {
		(Some(result), Some(ins)) => println!("New instructions: {:?},\n  yield: {:?}", ins, result),
		_ => println!("No single command can be found that by switching it can yield the program executes to completion."),
	};

	Ok(())
}
