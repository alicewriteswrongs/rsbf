use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn new(message: &str) -> Self {
        ParseError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub struct ParseErrors {
    pub errors: Vec<ParseError>,
}

impl ParseErrors {
    pub fn new() -> Self {
        ParseErrors { errors: vec![] }
    }
}

impl Display for ParseErrors {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Format your error message
        write!(f, "Custom error message")
    }
}

impl Error for ParseErrors {}

pub fn format_error_message(
    code: &str,
    line_number: usize,
    global_char_index: usize,
    error_message: &str,
) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let line = lines.get(line_number - 1).unwrap_or(&"");

    // Calculate local character position within the line
    let mut char_count = 0;
    for (i, c) in code.chars().enumerate() {
        if i == global_char_index {
            break;
        }
        char_count += 1;
        if c == '\n' {
            char_count = 0;
        }
    }
    let local_char_position = char_count;

    let mut message = format!("Line {}: {}\n", line_number, line);

    message.push_str("--------");
    for _ in 0..local_char_position {
        message.push('-');
    }
    message.push_str("^ Error here\n");
    message.push_str("\n");
    message.push_str(error_message);

    message
}
