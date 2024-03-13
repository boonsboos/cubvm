use crate::vm::cube::{self, Cube};
use crate::lang::codegen;

pub struct VM {
    stack: [Cube; 128],
    stack_pointer: usize,
    mem: [Cube; 8192], // 8K memory
    mem_pointer: usize,

    program_counter: usize,
    return_stack: Vec<usize>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: [Cube::new(); 128],
            stack_pointer: 0,
            mem: [Cube::default(); 8192],
            mem_pointer: 1, // 0 address = null

            program_counter: 1,
            return_stack: vec![],
        }
    }

    // this initializes the memory 
    pub fn interpret_code(&mut self, code: codegen::Code) {

        println!("{:?}", &code.movesets);

        assert!(code.movesets[0] == 0x00B0, "Corrupt bytecode!");

        let mut immediate = Cube::new();
        let mut current = Cube::new();

        while self.program_counter < code.movesets.len() {

            if self.return_stack.len() > 1024 {
                panic!("overflow of return stack (len > 1024)");
            }

            match code.movesets[self.program_counter] {
                0 => current.twist_u(),
                1 => current.twist_u_prime(),
                2 => current.twist_u2(),
                3 => current.twist_f(),
                4 => current.twist_f_prime(),
                5 => current.twist_f2(),
                6 => current.twist_r(),
                7 => current.twist_r_prime(),
                8 => current.twist_r2(),
                9 => current.twist_b(),
                10 => current.twist_b_prime(),
                11 => current.twist_b2(),
                12 => current.twist_l(),
                13 => current.twist_l_prime(),
                14 => current.twist_l2(),
                15 => current.twist_d(),
                16 => current.twist_d_prime(),
                17 => current.twist_d2(),

                18 => current.rotate_x(),
                19 => current.rotate_x_prime(),
                20 => current.rotate_x2(),
                21 => current.rotate_y(),
                22 => current.rotate_y_prime(),
                23 => current.rotate_y2(),
                24 => current.rotate_z(),
                25 => current.rotate_z_prime(),
                26 => current.rotate_z2(),

                0x003A => { // :
                    self.return_stack.push(self.program_counter);
                    self.program_counter = code.movesets[self.program_counter+1].into() // jump
                }
                0x003B => { // ;
                    self.interpret_cube(current, immediate);
                    current = Cube::new();
                    immediate = Cube::new();
                }
                0x003D => {
                    if self.stack[self.stack_pointer].sum_face(cube::U) > 0 {
                        self.return_stack.push(self.program_counter);
                        self.program_counter = code.movesets[self.program_counter+1].into() // jump conditionally
                    }
                }
                0x002C => { // ,
                    immediate = current;
                    current = Cube::new();
                }
                _ => {}
            }

            self.program_counter += 1;
        }
    }

    fn interpret_cube(&mut self, cube: Cube, immediate: Cube) {

        let opcode = cube.sum_face(cube::U);
        let imm = immediate.sum_face(cube::U);

        match opcode {
            0 => {} // NOP
            1 => { // PSH immediate
                if self.stack_pointer == 128 { panic!("cannot push full stack!") }

                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = immediate;
            }
            2 => { // POP
                if self.stack_pointer == 0 { panic!("cannot pop empty stack!") }

                self.stack[self.stack_pointer] = Cube::new();
                self.stack_pointer -= 1;
            }
            3 => self.mem[self.mem_pointer] = immediate, // MEM [mp] = imm
            4 => { // SMS [mp] -> [sp]
                let t = self.stack[self.stack_pointer];
                self.stack[self.stack_pointer] = self.mem[self.mem_pointer];
                self.mem[self.mem_pointer] = t; 
            }
            5 => { // SSM [sp] -> [mp]
                let t = self.mem[self.mem_pointer];
                self.mem[self.mem_pointer] = self.stack[self.stack_pointer];
                self.stack[self.stack_pointer] = t;   
            }
            6 => { // RET
                match self.return_stack.pop() {
                    Some(i) => self.program_counter = i,
                    None => panic!("popped empty return stack!"),
                }
            }
            _ => {}
        }

        // TODO: add labels
        //self.mem[self.mem_pointer] = opcode; // write the opcode to memory for use later
        self.mem_pointer += 1;
    }
}