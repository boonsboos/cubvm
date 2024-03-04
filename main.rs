mod vm;
mod lang;

use std::env;
use lang::compiler::compile;
use vm::vm::VM;

fn main() {
    let mut args = env::args_os();

    if args.len() < 1 { 
        println!("no arguments! bad!");
        return;
    }

    println!("{:?}", args);

    match args.nth(0) {
        Some(file) =>  {
            println!("{:?}", compile(file.into_string().unwrap()))
        }
        None => return,
    }

    let vm = VM::new();
}