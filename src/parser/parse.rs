use super::error::{format_error_message, ParseError, ParseErrors};

#[derive(Debug, Eq, PartialEq)]
pub enum BFCommand {
    DataPtrIncrement,
    DataPtrDecrement,
    Increment,
    Decrement,
    OutputByte,
    InputByte,
    ForwardGoto(usize),
    BackwardGoto(usize),
}

pub fn print_bf_commands(bf_commands: &[BFCommand]) {
    for (i, command) in bf_commands.iter().enumerate() {
        println!("{}: {:?}", i, command);
    }
}

pub fn parse(code: String) -> Result<Vec<BFCommand>, ParseErrors> {
    let mut bf_commands: Vec<BFCommand> = vec![];
    let mut forward_goto_indices: Vec<usize> = vec![];
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
                // find the index of the next ']' character in the 'code' string starting from
                // `index`
                let mut intervening_opening_brackets = 0;
                let mut intervening_command_count = 0;

                for (i, c) in code[(index)..].chars().enumerate() {
                    if i == 0 {
                        continue;
                    }
                    if c == '[' {
                        // there's another pair of [] opening, so we need to keep track of that so
                        // we know when the ']' we encounter is supposed to be matched with the one
                        // we started with
                        intervening_opening_brackets += 1;
                    }
                    if c == ']' {
                        // if this is 0 then we're on the same 'level' as where we started
                        if intervening_opening_brackets == 0 {
                            break;
                        } else {
                            intervening_opening_brackets -= 1;
                        }
                    }
                    // if c is a valid brainfuck command increment intervening_command_count
                    let brainfuck_commands = ['>', '<', '+', '-', '[', ']', ',', '.'];
                    if brainfuck_commands.iter().any(|&bf| bf == c) {
                        intervening_command_count += 1;
                    }
                }
                bf_commands.push(BFCommand::ForwardGoto(
                    bf_commands.len() + intervening_command_count + 1,
                ));
                forward_goto_indices.push(bf_commands.len() - 1);
            }
            ']' => {
                forward_goto_indices.pop().map_or_else(
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
            '\n' => line_number += 1,
            _ => {}
        }
    }

    if !forward_goto_indices.is_empty() {
        // there was an unmatched opening [
        parse_errors
            .errors
            .push(ParseError::new(&format_error_message(
                &code,
                line_number,
                forward_goto_indices.pop().unwrap(),
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

    #[test]
    fn test_paren_matching() {
        assert_eq!(
            parse(String::from("[]")).unwrap(),
            vec![BFCommand::ForwardGoto(1), BFCommand::BackwardGoto(0),]
        );
    }
    #[test]
    fn test_nested_paren_matching() {
        assert_eq!(
            parse(String::from("[+>[>]]")).unwrap(),
            vec![
                BFCommand::ForwardGoto(6),
                BFCommand::Increment,
                BFCommand::DataPtrIncrement,
                BFCommand::ForwardGoto(5),
                BFCommand::DataPtrIncrement,
                BFCommand::BackwardGoto(3),
                BFCommand::BackwardGoto(0),
            ]
        );
    }
}
