use wasm_bindgen::prelude::*;

mod interpreter;
mod parser;

#[wasm_bindgen]
pub fn run_brainfuck(code: String, f: &js_sys::Function) {
    let commands = parser::parse(code);

    let output_byte = |byte: u8| {
        let this = JsValue::null();
        let byte = JsValue::from(byte);
        f.call1(&this, &byte).unwrap();
    };

    interpreter::interpret(commands.unwrap(), &output_byte).unwrap();
}
