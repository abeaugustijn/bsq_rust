/// Parses the first line of a bsq file. Tries to parse the given line into an u32 for further
/// parsing.
///
/// @param {&str} line
///
/// @return {Result<u32, String>}

fn parse_height(line: &str) -> Result<usize, String> {
    return match line.parse::<usize>() {
        Ok(v) => Ok(v),
        Err(_) => Err(String::from("Invalid first line")),
    };
}

/// Parses a line. It checks for invalid characters and returns the width of the string for
/// validation and a vector representing the contents of the line.
///
/// @param {usize} index - used in the error message
///
/// @param {&str} line - reference to the line to check

fn parse_line(index: usize, line: &str) -> Result<(usize, Vec<u16>), String> {
    let len = line.len();

    let mut vec = Vec::with_capacity(len);
    for c in String::from(line).chars() {
        if c != '.' && c != 'o' {
            return Err(format!("Invalid char(s) in line {}", index));
        }
        vec.push(match c {
            '.' => 1,
            _ => 0, // This is the case for all other chars, which can only be 'o'
        });
    }
    Ok((line.len(), vec))
}

/// Validates the input file
///
/// @param {&String} contents - a reference to the contents of the input file
///
/// @return {Result<Vec<Vec<u16>>, String>}

pub fn validate_contents(contents: &String) -> Result<Vec<Vec<u16>>, String> {
    let mut iter = contents.lines().enumerate();

    // Parse the first line to get the given amount of lines in the files (height)
    let height = parse_height(match iter.next() {
        Some((_, h)) => h,
        None => return Err(String::from("File too short")),
    })?;

    let mut res = Vec::with_capacity(height);

    // First line should match the amount of lines
    if contents.lines().count() != height + 1 {
        return Err(String::from(
            "First line does not define the correct amount of lines in the file",
        ));
    }

    // Parse the second line to know what the len of every line should be
    let (width, line_vec) = match iter.next() {
        Some((_, h)) => parse_line(1, h)?,
        None => return Err(String::from("File too short")),
    };
    res.push(line_vec);

    for (i, l) in iter {
        let (line_width, line_vec) = parse_line(i, l)?;
        if line_width != width {
            return Err(format!("Invalid line width for line {}", i));
        }
        res.push(line_vec);
    }
    Ok(res)
}
