use crate::PasswordSpec;

#[test]
fn test_create_struct_regular() {
	const TEST_STR: &str = "11-13 a: abcde";
	let ps = PasswordSpec::new(TEST_STR).unwrap();
	assert_eq!(
		ps,
		PasswordSpec { first: 11, second: 13, test_char: 'a', passphrase: "abcde".to_string() }
	);
}

#[test]
fn test_err_on_invalid() {
	const TEST_STR: &str = "1a-3 a: abcde";
	let ps1 = PasswordSpec::new(TEST_STR);
	assert_eq!(ps1, Err("unknown phrase for parsing"));

	let ps2 = PasswordSpec::new("");
	assert_eq!(ps2, Err("unknown phrase for parsing"));
}

#[test]
fn test_min_max_count_01() {
	const TEST_STR: &str = "1-3 a: abcde";
	let ps = PasswordSpec::new(TEST_STR).unwrap();
	assert!(ps.match_min_max());
}

#[test]
// Test bound on "first" value
fn test_min_max_count_02() {
	const TEST_STR: &str = "2-3 a: abcde";
	let ps = PasswordSpec::new(TEST_STR).unwrap();
	assert!(ps.match_min_max() == false);
}

#[test]
// Test bound on "second" value
fn test_min_max_count_03() {
	const TEST_STR: &str = "0-1 a: aabcde";
	let ps = PasswordSpec::new(TEST_STR).unwrap();
	assert!(ps.match_min_max() == false);
}

#[test]
fn test_match_one_pos_of_one_01() {
	const TEST_STR: &str = "1-3 a: abcde";
	let ps = PasswordSpec::new(TEST_STR).unwrap();
	assert!(ps.match_one_pos());
}

#[test]
fn test_match_one_pos_of_one_02() {
	const TEST_STR: &str = "1-3 a: bbade";
	let ps = PasswordSpec::new(TEST_STR).unwrap();
	assert!(ps.match_one_pos());
}

#[test]
fn test_match_one_pos_of_two() {
	const TEST_STR: &str = "1-3 a: abade";
	let ps = PasswordSpec::new(TEST_STR).unwrap();
	assert!(ps.match_one_pos() == false);
}

#[test]
fn test_match_one_pos_of_zero() {
	const TEST_STR: &str = "1-3 a: bbsde";
	let ps = PasswordSpec::new(TEST_STR).unwrap();
	assert!(ps.match_one_pos() == false);
}

#[test]
fn test_match_one_pos_out_of_bound_01() {
	const TEST_STR: &str = "1-12 a: bbsde";
	let ps = PasswordSpec::new(TEST_STR).unwrap();
	assert!(ps.match_one_pos() == false);
}

#[test]
fn test_match_one_pos_out_of_bound_02() {
	const TEST_STR: &str = "1-12 a: absde";
	let ps = PasswordSpec::new(TEST_STR).unwrap();
	assert!(ps.match_one_pos());
}
