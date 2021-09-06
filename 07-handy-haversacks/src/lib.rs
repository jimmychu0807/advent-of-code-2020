use std::collections::BTreeMap;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
	static ref IGNORE_RES: [regex::Regex; 1] = [
		Regex::new(r"contain no other bags\.$").unwrap(),
	];

	static ref KEY_RE: Regex = Regex::new(r"^\s*(\w+\s+\w+)\s+bags\s*$")
		.unwrap();

	static ref NUM_BAG_RE: Regex = Regex::new(r"^\s*(\d+)\s+(\w+\s+\w+)\s+bags?\.?$")
		.unwrap();
}

#[derive(Debug)]
pub struct Haversacks {
	bag_rules: BTreeMap<String, Vec<(u32, String)>>
}

impl Haversacks {
	pub fn setup(rule_arr: Vec<&str>) -> Self {
		let mut bag_rules = BTreeMap::<String, Vec<(u32, String)>>::new();

		'rules: for rule_str in rule_arr.iter() {
			for ignore_re in IGNORE_RES.iter() {
				if ignore_re.captures(rule_str).is_some() {
					continue 'rules;
				}
			}

			let mut main_splitted = rule_str.split("contain");
			let key_str = main_splitted.nth(0).unwrap(); // `nth()` will consume the element
			let bags_str = main_splitted.nth(0).unwrap();

			let key_cap = KEY_RE.captures(key_str).expect("should returned key");
			let key = key_cap.get(1).unwrap().as_str();

			let rule_vec: Vec<_> = bags_str.split(",").map(|bag_str| {
				let num_bag_cap = NUM_BAG_RE.captures(bag_str)
					.expect("should capture num and bag string");

				let num: u32 = num_bag_cap.get(1).expect("should capture a number")
					.as_str().parse::<u32>().expect("should be convertible to a u32 integer");

				let bag = num_bag_cap.get(2).expect("should capture a bag style").as_str();

				(num, bag.to_string())
			}).collect();

			bag_rules.insert(key.to_string(), rule_vec);
		}

		Haversacks { bag_rules }
	}

	pub fn find_contained_qty(&self, target: &str) -> u32 {
		self.bag_rules.keys()
			.filter(|bag_key| bag_key.as_str() != target)
			.map(|bag_key| self.contain_eventually(bag_key, target))
			.fold(0, |acc, val| {
				match val {
					true => acc + 1,
					false => acc
				}
			})
	}

	fn contain_eventually(&self, bag_key: &str, target: &str) -> bool {
		if bag_key == target {
			true
		}
		else if let Some(rule_vec) = self.bag_rules.get(bag_key) {
			rule_vec.iter()
				.map(|(_, bag)| self.contain_eventually(bag, target))
				.fold(false, |acc, result| acc || result)
		} else {
			false
		}
	}
}

