use std::fs;

pub fn read_file(path: &str) -> Result<Vec<String>, &'static str> {
	let contents = fs::read_to_string(path)
		.map_err(|_| "File cannot be read")?;

	let lines: Vec<String> = contents.split('\n')
		.map(|line| line.trim().to_string())
		.collect();

	// Trim away the empty lines and the beginning and at the end
	let mut state = false;
	let mut start_index = 0;
	let mut end_index = 0;

	// This is to handle line you need to jump over when incrementing end_index again
	let mut skipped = 0;

	for line in lines.iter() {
		if !state && line.is_empty() {
			start_index += 1;
			end_index = start_index;
		} else if !state && !line.is_empty() {
			state = true;
			end_index = start_index;
		} else if state && !line.is_empty() {
			end_index += skipped + 1;
			skipped = 0;
		} else {
			// state && line.is_empty()
			skipped += 1;
		}
	}

	let content_lines: Vec<String> = lines[start_index..=end_index]
		.iter()
		.map(|s| s.clone())
		.collect::<Vec<_>>();

	Ok(content_lines)
}

pub fn vec_string_to_str(lines: &[String]) -> Vec<&str> {
	lines.iter().map(|s| &**s).collect()
}

pub fn read_file_and_group(path: &str) -> Result<Vec<Vec<String>>, &'static str> {
	let lines = read_file(path)?;

	let mut total_set = Vec::new();
	let mut set = Vec::new();

	for line in lines.iter() {
		if line.is_empty() && !set.is_empty() {
			total_set.push(set.clone());
			set = Vec::new();

		} else if !line.is_empty() {
			set.push(line.clone());
		}
	}

	// This is the last set
	if !set.is_empty() {
		total_set.push(set.clone());
	}

	Ok(total_set)
}
