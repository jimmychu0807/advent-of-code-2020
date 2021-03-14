use report_repair::ReportRepair;
use shared::{read_file};

const INPUT_PATH: &str = "01-report-repair/data/input.dat";
const TARGET: u64 = 2020;

fn main() {
	let input_lines = read_file(INPUT_PATH).unwrap();
	let input_nums: Vec<u64> = input_lines.into_iter()
		.filter_map(|l| l.parse::<u64>().ok())
		.collect();

	let rr = ReportRepair::new(input_nums);

	if let Some((v1, v2)) = rr.sum_two_to_target(TARGET) {
		println!("The sum of ({}, {}): {}", v1, v2, v1 + v2);
		println!("The product of ({}, {}): {}", v1, v2, v1 * v2);
	}

	if let Some((v1, v2, v3)) = rr.sum_three_to_target(TARGET) {
		println!("The sum of ({}, {}, {}): {}", v1, v2, v3, v1 + v2 + v3);
		println!("The product of ({}, {}, {}): {}", v1, v2, v3, v1 * v2 * v3);
	}
}
