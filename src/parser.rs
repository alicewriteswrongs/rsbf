use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BFCommand {
    DataPtrIncrement,
    DataPtrDecrement,
    Increment,
    Decrement,
    OutputByte,
    InputByte,
    ForwardGoto,
    BackwardGoto(usize),
}

#[derive(Debug)]
pub struct ParseError {
    message: String
}

impl ParseError {
    fn new(message: &str) -> Self {
        ParseError {
            message: message.to_string()
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseError {}

fn format_error_message(code: &str, line_number: usize, global_char_index: usize, error_message: &str) -> String {
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




pub fn parse(code: String) -> Result<Vec<BFCommand>, ParseError> {
        let mut bf_commands: Vec<BFCommand> = vec![];
        let mut goto_indices: Vec<usize> = vec![];
        let mut line_number = 1;

        if code.is_empty() {
            return Err(ParseError::new("Empty input"));
        }

        for (index, c ) in code.chars().enumerate() {
            match c {
                '>' => bf_commands.push(BFCommand::DataPtrIncrement),
                '<' => bf_commands.push(BFCommand::DataPtrDecrement),
                '+' => bf_commands.push(BFCommand::Increment),
                '-' => bf_commands.push(BFCommand::Decrement),
                '[' => {
                    bf_commands.push(BFCommand::ForwardGoto);
                    goto_indices.push(index);
                },
                ']' => {
                    let val = goto_indices.pop();

                    if val.is_none() {
                        // parsing error, unmatched closing ]
                        return Err(ParseError::new(
                            &format_error_message(&code, line_number, index,
                                "Found a closing ']' without an opening '['")
                        ));
                    } else {
                        let index = val.unwrap();
                        bf_commands.push(BFCommand::BackwardGoto(index));
                    }
                },
                '.' => bf_commands.push(BFCommand::OutputByte),
                ',' => bf_commands.push(BFCommand::InputByte),
                '\n' => line_number = line_number + 1,
                _ => {},
            }
        }
        Ok(bf_commands)
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_characters() {
        assert_eq!( parse(String::from("+")).unwrap(), vec![BFCommand::Increment]);
        assert_eq!( parse(String::from("-")).unwrap(), vec![BFCommand::Decrement]);
        assert_eq!( parse(String::from(">")).unwrap(), vec![BFCommand::DataPtrIncrement]);
        assert_eq!( parse(String::from("<")).unwrap(), vec![BFCommand::DataPtrDecrement]);
        assert_eq!( parse(String::from(".")).unwrap(), vec![BFCommand::OutputByte]);
        assert_eq!( parse(String::from(",")).unwrap(), vec![BFCommand::InputByte]);
    }

    #[test]
    fn test_err_empty_input() {
        assert_eq!( parse(String::from("")).unwrap_err().message, "Empty input");
    }

    #[test]
    fn test_unmatch_closing_parens1() {
        let val = parse(String::from("++.,<<>>]")).unwrap_err();
        println!("{}", val);
        assert_eq!( parse(String::from("]")).unwrap_err().message, "Empty input");
    }


    #[test]
    fn test_unmatch_closing_parens() {
        let val = parse(String::from("]")).unwrap_err();
        println!("{}", val);
        assert_eq!( parse(String::from("]")).unwrap_err().message, "Empty input");
    }

}
