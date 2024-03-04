use crate::vm::cube;
use crate::lang::codegen;

pub struct VM {
    stack: Vec<Vec<Vec<cube::Cube>>>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: vec![vec![vec![cube::Cube::new()]]],
        }
    }

    pub fn interpret_code(code: codegen::Code) {
        
    }
}