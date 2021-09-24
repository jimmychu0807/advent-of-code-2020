use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Default, Clone)]
pub struct Compiler {
	ins: Vec<String>,
	debug: bool,
}

#[derive(Debug, Clone)]
pub enum ExitStatus {
	Okay = 0,
	Repeated = 1,
	JumpOutOfBound = 2
}

#[derive(Debug, Clone)]
pub struct ExecutedResult {
	pub status: ExitStatus,
	pub acc: i64,
	pub executed: Vec<u64>
}

type Delta = (i64, i64);

impl Compiler {
	pub fn new(ins: &Vec<String>, debug: bool) -> Compiler {
		Compiler {
			ins: ins.clone(),
			debug
		}
	}

	pub fn execute(&self) -> Result<ExecutedResult, String> {
		// current cursor
		let mut cursor: i64 = 0;
		// accumulator
		let mut acc: i64 = 0;
		let mut executed: Vec<u64> = vec![0; self.ins.len()];
		let mut status = ExitStatus::Okay;

		loop {
			// terminate condition
			if cursor < 0 || cursor >= (self.ins.len() as i64) {
				status = ExitStatus::JumpOutOfBound;
				break;
			} else if executed[cursor as usize] > 0 {
				status = ExitStatus::Repeated;
				break;
			}

			let (acc_delta, cursor_delta) = execute_one_cmd(&self.ins[cursor as usize], self.debug)?;

			// Update book-keeping variables
			executed[cursor as usize] += 1;
			acc += acc_delta;
			cursor += cursor_delta;

			if self.debug {
				println!("acc: {}, executed: {:?}", acc, executed);
			}

			if cursor_delta == 1 && cursor == (self.ins.len() as i64) {
				// successfully executed the last statement of the program
				break;
			}
		}
		Ok(ExecutedResult { status, acc, executed })
	}
}

fn execute_one_cmd(one_cmd: &str, debug: bool) -> Result<Delta, String> {
	lazy_static! {
		static ref SPACES: Regex = Regex::new(r"\s+").unwrap();
	}

	if debug {
		println!("exe: {}", one_cmd);
	}

	let to_val = |val_str: &str| -> Result<i64, String> {
		let sym = &val_str[0..1];
		let val = &val_str[1..].parse::<i64>()
			.map_err(|_| format!("Cannot parse value: {}", val_str))?;

		match sym {
			"+" => Ok(*val),
			"-" => Ok(*val * -1),
			_ => Err(format!("Cannot parse symbol: {}", val_str)),
		}
	};

	let splitted = SPACES.split(one_cmd).collect::<Vec<_>>();
	let cmd = splitted[0];
	let val = to_val(splitted[1])?;

	let acc_delta = match cmd {
		"nop" | "jmp" => Ok(0),
		"acc" => Ok(val),
		_ => Err(format!("Unknown cmd: {}", cmd))
	}?;

	let cursor_delta = match cmd {
		"nop" | "acc" => Ok(1),
		"jmp" => Ok(val),
		_ => Err(format!("Unknown cmd: {}", cmd))
	}?;

	Ok((acc_delta, cursor_delta))
}
