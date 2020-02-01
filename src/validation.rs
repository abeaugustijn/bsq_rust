/// Validates the first line of a bsq file. Tries to parse the given line into an u32 for further
/// parsing.
///
/// @param {&str} line
///
/// @return {Result<u32, String>}

fn validate_size(line: &str) -> Result<u32, String> {
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
    let mut height = validate_size(iter.next()?.1)?;
    let mut width = validate_width(

    for (i, l) in contents.lines().enumerate() {
        match i {
            0 => height = validate_size(l)?,
            1 => width = validate_width(i, l)?,
            _ => {
                if validate_width(i, l)? != width {
                    return Err(format!("Invalid line width for line {}", i));
                }
            }
        }
    }
    Ok(())
}
