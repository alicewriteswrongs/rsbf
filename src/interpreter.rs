use crate::parser::BFCommand;
use std::io::{self, Read, Write};

#[derive(Debug)]
struct BFDataBuffer {
    inner: Vec<u8>,
}

impl BFDataBuffer {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn get(&self, index: usize) -> u8 {
        let val = self.inner.get(index).unwrap_or(&0);
        return *val;
    }

    fn set(&mut self, index: usize, value: u8) {
        if index >= self.inner.len() {
            self.inner.resize(index + 1, 0);
        }
        self.inner[index] = value;
    }
}

pub fn interpret(commands: Vec<BFCommand>) -> anyhow::Result<()> {
    let mut data_buffer = BFDataBuffer::new();
    let mut data_pointer: usize = 0;
    let mut instruction_pointer: usize = 0;
    let mut stdout = io::stdout();

    while let Some(command) = commands.get(instruction_pointer) {
        match command {
            BFCommand::DataPtrIncrement => {
                data_pointer += 1;
                instruction_pointer += 1;
            }
            BFCommand::DataPtrDecrement => {
                if data_pointer > 0 {
                    data_pointer -= 1;
                };
                instruction_pointer += 1;
            }
            BFCommand::Increment => {
                data_buffer.set(data_pointer, data_buffer.get(data_pointer) + 1);
                instruction_pointer += 1;
            }
            BFCommand::Decrement => {
                data_buffer.set(data_pointer, data_buffer.get(data_pointer) - 1);
                instruction_pointer += 1;
            }
            BFCommand::OutputByte => {
                stdout.write(&[data_buffer.get(data_pointer)])?;
                stdout.flush()?;
                instruction_pointer += 1;
            }
            BFCommand::InputByte => {
                let mut buffer = [0; 1];
                io::stdin().read_exact(&mut buffer).unwrap();
                let byte = buffer[0];
                data_buffer.set(data_pointer, byte);
                instruction_pointer += 1;
            }
            BFCommand::ForwardGoto(forward_index) => {
                if data_buffer.get(data_pointer) == 0 {
                    instruction_pointer = *forward_index;
                }
                instruction_pointer += 1;
            }
            BFCommand::BackwardGoto(back_index) => {
                if data_buffer.get(data_pointer) != 0 {
                    instruction_pointer = *back_index;
                }
                instruction_pointer += 1;
            }
        }
    }

    Ok(())
}
