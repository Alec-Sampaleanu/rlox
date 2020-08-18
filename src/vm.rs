use crate::{chunk, value, compiler};

pub enum InterpretResult {
    InterpretOK,
    InterpretCompileError,
    InterpretRuntimeError,
}

#[derive(Debug)]
pub struct VM<> {
    pub chunk: Option<chunk::Chunk>,
    pub stack: Vec<value::Value>,
}

impl VM {
    pub fn init() -> VM {
        VM { chunk: None, stack: Vec::new() }
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        compiler::compile(source);
        InterpretResult::InterpretOK
    }

    pub fn run(&mut self) -> InterpretResult {
        let chunk_ref = self.chunk.as_ref().unwrap();

        for (pos, code) in chunk_ref.code.iter().enumerate() {

            // Debug code
            print!("          ");
            for val in &self.stack {
                print!("[{}]", val)
            }
            println!();
            chunk_ref.disassemble_instruction(pos, &code);

            // Prod code
            match code {
                chunk::OpCode::OpReturn => {
                    println!("{}", self.stack.pop().unwrap());
                    return InterpretResult::InterpretOK;
                }
                chunk::OpCode::OpConstant(i) => {
                    let constant = chunk_ref.constants.get(*i);
                    self.stack.push(*constant);
                }
                chunk::OpCode::OpNegate => {
                    let val = self.stack.pop().unwrap();
                    self.stack.push(-val)
                }
                chunk::OpCode::OpAdd => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                },
                chunk::OpCode::OpSubtract => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                },
                chunk::OpCode::OpMultiply => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                },
                chunk::OpCode::OpDivide => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                },
            };
        }
        InterpretResult::InterpretCompileError
    }
}