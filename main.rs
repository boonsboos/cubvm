mod vm;

use std::env;
use vm::vm::VM;

fn main() {
    let args = env::args_os();

    if args.len() < 1 { 
        return;
    }

    let vm = VM::new();
}