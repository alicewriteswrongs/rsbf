mod interpreter;
mod parser;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Filename to read
    #[clap(name = "filename")]
    filename: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let contents = std::fs::read_to_string(&args.filename)?;

    let commands = parser::parse(contents)?;
    println!("{:?}", commands);
    interpreter::interpret(commands)?;
    Ok(())
}
