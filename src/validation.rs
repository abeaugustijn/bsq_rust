/// Validates the first line of a bsq file. Tries to parse the given line into an u32 for further
/// parsing.
///
/// @param {&str} line
///
/// @return {Result<u32, String>}

fn validate_height(line: &str) -> Result<u32, String> {
	return match line.parse::<u32>() {
		Ok(v) => Ok(v),
		Err(_) => Err(String::from("Invalid first line")),
	};
}

/// Validates a line. It checks for invalid characters and returns the width of the string for
/// validation with the width of the first line.
///
/// @param {usize} index - used in the error message
///
/// @param {&str} line - reference to the line to check

fn validate_width(index: usize, line: &str) -> Result<u32, String> {
	for c in String::from(line).chars() {
		if c != '.' && c != 'o' {
			return Err(format!("Invalid char(s) in line {}", index));
		}
	}
	Ok(line.len() as u32)
}

pub fn validate_contents(contents: &String) -> Result<(), String> {
	let mut iter = contents.lines().enumerate();

	// Parse the first line to get the given amount of lines in the files (height)
	let height = validate_height(match iter.next() {
		Some((_, h)) => h,
		None => return Err(String::from("File too short")),
	})?;

	// First line should match the amount of lines
	if contents.lines().count() as u32 != height + 1 {
		return Err(String::from(
			"First line does not define the correct amount of lines in the file",
		));
	}

	// Get the len of the second line to know what the len of every line should be
	let width: u32 = match iter.next() {
		Some((_, h)) => h.len() as u32,
		None => return Err(String::from("File too short")),
	};

	for (i, l) in contents.lines().enumerate() {
		if validate_width(i, l)? != width {
			return Err(format!("Invalid line width for line {}", i));
		}
	}
	Ok(())
}
