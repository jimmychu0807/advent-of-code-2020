use crate::*;

#[test]
fn test_valid01() {
	let valid = vec![
		"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
		"byr:1937 iyr:2017 cid:147 hgt:183cm"
	];

	let builder = Passport.builder();
	for line in valid.iter() {
		builder = builder.process(line).unwrap();
	}
	assert_eq!(builder.build(), Passport {
		byr: 1937,
		eyr: 2020,
		iyr: 2017,
		ecl: "gry",
		hcl: "#fffffd",
		hgt: "183cm",
		pid: "860033327",
		cid: Some("147")
	});
}
