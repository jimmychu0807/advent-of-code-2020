use regex::Regex;

const test_data: [&str; 3] = [
	"1-3 a: abcde",
	"1-3 b: cdefg",
	"2-9 c: cccccccc"
];

fn main() {
	let count = test_data
		.iter()
		.filter(|phrase| valid_password(phrase))
		.collect::<Vec<&&str>>()
		.len();

	println!("count: {}", count);
}

fn valid_password(phrase: &str) -> bool {
	let regex: Regex = Regex::new(r"(\d+)-(\d+)\s+(.):\s+(\S)")
		.unwrap();

}
