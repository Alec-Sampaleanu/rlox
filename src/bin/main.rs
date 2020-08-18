extern crate rlox;

use std::env;

use rlox::{run_file, run_prompt};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() > 1 {
        eprintln!("Usage: lox [script]");
    } else if args.len() == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}