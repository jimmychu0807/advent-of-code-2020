use std::collections::BTreeMap;

pub struct Haversacks {
	rules: BTreeMap<String, Vec<(String, u32)>>
}

impl Haversacks {
	pub fn find_outer_qty(&self, bag: &str) -> u32 {
		1
	}
}

pub fn build_rules(rule_arr: Vec<&str>) -> Haversacks {
	Haversacks {
		rules: BTreeMap::new()
	}
}
