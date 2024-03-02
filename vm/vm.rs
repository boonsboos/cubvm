use crate::vm::cube;

pub struct VM {
    stack: Vec<Vec<Vec<cube::Cube>>>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: vec![vec![vec![cube::Cube::new()]]],
        }
    }

    /// pushes the cube to the top of the first stack.
    pub fn push_normally(&mut self, c: cube::Cube) {
        self.stack[0][0].push(c);
    }
}