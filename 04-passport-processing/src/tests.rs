use crate::*;

#[test]
fn test_valid_from_str() {
	let valid = vec!(
		"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
		"byr:1937 iyr:2017 cid:147 hgt:183cm"
	);

	let mut passport = Passport::new();

	assert_eq!(passport.valid().is_ok(), false);

	for line in valid.iter() {
		passport = passport.process(line).unwrap();
	}

	assert_eq!(passport.valid().is_ok(), true);

	// Test fields in the passport
	assert_eq!(passport.fields.get("byr"), Some(&1937.into()));
	assert_eq!(passport.fields.get("eyr"), Some(&2020.into()));
	assert_eq!(passport.fields.get("ecl"), Some(&"gry".into()));
}

#[test]
fn test_valid_from_file() {
	let filepath = "data/tests/test_valid01.dat";
	let passports = read_from_file(filepath).unwrap();
	assert_eq!(passports.len(), 1);
	let passport = &passports[0];

	assert_eq!(passport.valid().is_ok(), true);

	// Test fields in the passport
	assert_eq!(passport.fields.get("byr"), Some(&1937.into()));
	assert_eq!(passport.fields.get("eyr"), Some(&2020.into()));
	assert_eq!(passport.fields.get("ecl"), Some(&"gry".into()));
}

#[test]
fn test_valid_from_file_multiple_passports() {
	let filepath = "data/tests/test_valid02_two.dat";
	let passports = read_from_file(filepath).unwrap();
	assert_eq!(passports.len(), 2);
	let passport1 = &passports[0];
	let passport2 = &passports[1];

	assert_eq!(passport1.valid().is_ok(), true);
	assert_eq!(passport2.valid().is_ok(), true);

	// Test fields in the passport
	assert_eq!(passport1.fields.get("pid"), Some(&"860033327".into()));
	assert_eq!(passport2.fields.get("pid"), Some(&"860033326".into()));
}

#[test]
fn test_invalid_from_str() {
	let invalid = vec![
		"iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
		"hcl:#cfa07d byr:1929"
	];

	let mut passport = Passport::new();
	for line in invalid.iter() {
		passport = passport.process(line).unwrap();
	}

	match passport.valid() {
		Ok(_) => { assert!(false) }
		Err(err_vec) => { assert_eq!(err_vec, vec!(PassportInvalid::from("hgt"))) }
	};
}
