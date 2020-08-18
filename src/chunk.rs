use crate::value;

#[derive(Debug)]
pub enum OpCode {
    OpConstant(usize),
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: value::ValueArray,
    pub lines: Vec<u32>,
}

impl Chunk {
    pub fn init() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: value::ValueArray::init(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, code: OpCode, line: u32) {
        self.code.push(code);
        self.lines.push(line);
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        for (pos, code) in self.code.iter().enumerate() {
            self.disassemble_instruction(pos, code);
        }
    }

    pub fn disassemble_instruction(&self, pos: usize, code: &OpCode) {
        let line = self.get_line(pos);
        match code {
            OpCode::OpReturn => println!("{} OP_RETURN", line),
            OpCode::OpConstant(i) => {
                let val = self.constants.get(*i);
                println!("{} OP_CONSTANT   {} '{}'", line, i, val);
            }
            OpCode::OpNegate => println!("{} OP_NEGATE", line),
            OpCode::OpAdd => println!("{} OP_ADD", line),
            OpCode::OpSubtract => println!("{} OP_SUBTRACT", line),
            OpCode::OpMultiply => println!("{} OP_MULTIPLY", line),
            OpCode::OpDivide => println!("{} OP_DIVIDE", line),
        };
    }

    pub fn add_constant(&mut self, value: value::Value) -> usize {
        self.constants.write(value) - 1
    }

    pub fn get_line(&self, instruction: usize) -> &u32 {
        self.lines.get(instruction).unwrap()
    }
}