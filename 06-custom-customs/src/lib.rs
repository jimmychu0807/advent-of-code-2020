use std::collections::HashSet;

pub fn union_set(lines: &[&str]) -> HashSet<char> {
	lines
		.iter()
		.map(|l| l.chars().collect::<HashSet<char>>())
		.fold(HashSet::new(), |acc: HashSet<char>, set|
			acc.union(&set).map(|c| *c).collect()
		)
}

pub fn intersection_set(lines: &[&str]) -> HashSet<char> {
	lines
		.iter()
		.map(|l| l.chars().collect::<HashSet<char>>())
		.fold(None, |acc: Option<HashSet<char>>, set|
			match acc {
				Some(acc_set) => Some(acc_set.intersection(&set).map(|c| *c).collect()),
				None => Some(set),
			}
		)
		.unwrap_or(HashSet::new())
}
