mod vm;
mod lang;

use std::{env, time::Instant};
use lang::compiler::compile;
use vm::vm::VM;

fn main() {
    let mut args = env::args_os();

    if args.len() < 1 { 
        println!("you must supply the path to your program as an argument");
        return;
    }

    let start = Instant::now();

    let code = match args.nth(1) {
        Some(file) =>  {
            compile(file.into_string().unwrap())
        }
        None => return,
    };

    println!("Compiled in {:?}", start.elapsed());

    let mut vm = VM::new();
    vm.interpret_code(code);
}