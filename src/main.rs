mod interpreter;
mod parser;

use clap::Parser;
use std::io::{self, Write};

#[derive(Parser, Debug)]
struct Args {
    /// Filename to read
    #[clap(name = "filename")]
    filename: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let contents = std::fs::read_to_string(args.filename)?;

    let commands = parser::parse(contents)?;
    // parser::print_bf_commands(&commands);

    fn output_byte(byte: u8) {
        let mut stdout = io::stdout();
        stdout.write_all(&[byte]).unwrap();
        stdout.flush().unwrap();
    }

    interpreter::interpret(commands, &output_byte)?;
    Ok(())
}
