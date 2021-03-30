use crate::*;

#[test]
fn test_valid_from_str() {
	let valid = vec![
		"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
		"byr:1937 iyr:2017 cid:147 hgt:183cm"
	];

	let mut builder = Passport::builder();
	for line in valid.iter() {
		builder = builder.process(line).unwrap();
	}
	assert_eq!(builder.build(), Ok(Passport {
		byr: 1937,
		eyr: 2020,
		iyr: 2017,
		ecl: "gry".to_string(),
		hcl: "#fffffd".to_string(),
		hgt: "183cm".to_string(),
		pid: "860033327".to_string(),
		cid: Some("147".to_string())
	}));
}

#[test]
fn test_valid_from_file() {
	let filepath = "data/tests/test_valid01.dat";
	let (passports, errors) = read_from_file(filepath).unwrap();
	assert_eq!(passports.len(), 1);
	assert_eq!(passports[0], Passport {
		byr: 1937,
		eyr: 2020,
		iyr: 2017,
		ecl: "gry".to_string(),
		hcl: "#fffffd".to_string(),
		hgt: "183cm".to_string(),
		pid: "860033327".to_string(),
		cid: Some("147".to_string())
	});
	assert_eq!(errors.len(), 0);
}

#[test]
fn test_invalid_from_str() {
	let invalid = vec![
		"iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
		"hcl:#cfa07d byr:1929"
	];

	let mut builder = Passport::builder();
	for line in invalid.iter() {
		builder = builder.process(line).unwrap();
	}
	assert_eq!(builder.build(), Err("hgt missing"));
}

#[test]
fn test_invalid_from_file() {
	let filepath = "data/tests/test_invalid01.dat";
	let (passports, errors) = read_from_file(filepath).unwrap();
	assert_eq!(passports.len(), 0);
	assert_eq!(errors.len(), 1);
}
