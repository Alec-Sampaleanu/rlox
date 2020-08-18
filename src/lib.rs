use std::fs;
use rustyline;

use crate::chunk::OpCode::*;
use std::process::exit;

mod compiler;
mod scanner;
mod vm;
mod value;
mod chunk;

pub fn run_file(filename: &str) {
    let mut vm = vm::VM::init();

    let source = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    match vm.interpret(source) {
        vm::InterpretResult::InterpretCompileError => exit(65),
        vm::InterpretResult::InterpretRuntimeError => exit(70),

        _ => { exit(0) }
    };
}

pub fn run_prompt() {
    let mut vm = vm::VM::init();
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => println!("Line: {}", line),
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };
    }
}