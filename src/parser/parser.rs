use super::error::{format_error_message, ParseError, ParseErrors};

#[derive(Debug, Eq, PartialEq)]
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

pub fn parse(code: String) -> Result<Vec<BFCommand>, ParseErrors> {
    let mut bf_commands: Vec<BFCommand> = vec![];
    let mut goto_indices: Vec<usize> = vec![];
    let mut line_number = 1;
    let mut parse_errors = ParseErrors::new();

    if code.is_empty() {
        parse_errors.errors.push(ParseError::new("Empty input"));
    }

    for (index, c) in code.chars().enumerate() {
        match c {
            '>' => bf_commands.push(BFCommand::DataPtrIncrement),
            '<' => bf_commands.push(BFCommand::DataPtrDecrement),
            '+' => bf_commands.push(BFCommand::Increment),
            '-' => bf_commands.push(BFCommand::Decrement),
            '[' => {
                bf_commands.push(BFCommand::ForwardGoto);
                goto_indices.push(index);
            }
            ']' => {
                goto_indices.pop().map_or_else(
                    || {
                        parse_errors
                            .errors
                            .push(ParseError::new(&format_error_message(
                                &code,
                                line_number,
                                index,
                                "Found a closing ']' without an opening '['",
                            )));
                    },
                    |x| {
                        bf_commands.push(BFCommand::BackwardGoto(x));
                    },
                );
            }
            '.' => bf_commands.push(BFCommand::OutputByte),
            ',' => bf_commands.push(BFCommand::InputByte),
            '\n' => line_number +=1,
            _ => {}
        }
    }

    if !goto_indices.is_empty() {
        // there was an unmatched opening [
        parse_errors
            .errors
            .push(ParseError::new(&format_error_message(
                &code,
                line_number,
                goto_indices.pop().unwrap(),
                "Found an opening '[' without a closing ']'",
            )));
    }

    if parse_errors.errors.is_empty() {
        Ok(bf_commands)
    } else {
        Err(parse_errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_characters() {
        assert_eq!(
            parse(String::from("+")).unwrap(),
            vec![BFCommand::Increment]
        );
        assert_eq!(
            parse(String::from("-")).unwrap(),
            vec![BFCommand::Decrement]
        );
        assert_eq!(
            parse(String::from(">")).unwrap(),
            vec![BFCommand::DataPtrIncrement]
        );
        assert_eq!(
            parse(String::from("<")).unwrap(),
            vec![BFCommand::DataPtrDecrement]
        );
        assert_eq!(
            parse(String::from(".")).unwrap(),
            vec![BFCommand::OutputByte]
        );
        assert_eq!(
            parse(String::from(",")).unwrap(),
            vec![BFCommand::InputByte]
        );
    }

    #[test]
    fn test_err_empty_input() {
        assert_eq!(
            parse(String::from(""))
                .unwrap_err()
                .errors
                .pop()
                .unwrap()
                .message,
            "Empty input"
        );
    }

    #[test]
    fn test_unmatch_closing_parens() {
        assert_eq!(
            parse(String::from("++.,<<>>]")).unwrap_err().errors.pop().unwrap().message,
            "Line 1: ++.,<<>>]\n----------------^ Error here\n\nFound a closing ']' without an opening '['"
        );
    }

    #[test]
    fn test_unmatch_closing_leading_parens() {
        assert_eq!(
            parse(String::from("]"))
                .unwrap_err()
                .errors
                .pop()
                .unwrap()
                .message,
            "Line 1: ]\n--------^ Error here\n\nFound a closing ']' without an opening '['"
        );
    }

    #[test]
    fn test_unmatch_opening_parens() {
        assert_eq!(
            parse(String::from("["))
                .unwrap_err()
                .errors
                .pop()
                .unwrap()
                .message,
            "Line 1: [\n--------^ Error here\n\nFound an opening '[' without a closing ']'"
        );
    }
}
