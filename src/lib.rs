use wasm_bindgen::prelude::*;

mod interpreter;
mod parser;

#[wasm_bindgen]
pub fn run_brainfuck(code: String) {
    let commands = parser::parse(code);
    interpreter::interpret(commands.unwrap()).unwrap();
}
