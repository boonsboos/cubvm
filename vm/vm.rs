use crate::vm::cube;
use crate::lang::codegen;

use super::cube::Cube;

pub struct VM {
    stack: [u8; 128],
    stack_pointer: usize,
    mem: [cube::Cube; 8192], // 8K memory
    mem_pointer: usize,
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: [0; 128],
            stack_pointer: 0,
            mem: [cube::Cube::default(); 8192],
            mem_pointer: 1, // 0 address = null
        }
    }

    // this initializes the memory 
    pub fn interpret_code(&mut self, code: codegen::Code) {

        assert!(code.movesets[0] == 0xB0u8, "Corrupt bytecode!");

        let mut i: usize = 1;
        while i < code.movesets.len() {
            let mut immediate = Cube::new();
            let mut current = Cube::new();

            match code.movesets[i] {
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

                b';' => {
                    self.interpret_cube(current, immediate);
                }
                b',' => {
                    immediate = current;
                    current = Cube::new();
                }
                _ => {}
            }

            i += 1;
        }
    }

    fn interpret_cube(&mut self, cube: Cube, immediate: Cube) {
        let u = cube.faces[cube::U];

        let opcode = u[0] + u[1] + u[2] + u[3] + u[4] + u[5] +u[6] + u[7] + u[8];

        let u2 = immediate.faces[cube::U];
        let imm = u2[0] + u2[1] + u2[2] + u2[3] + u2[4] + u2[5] +u2[6] + u2[7] + u2[8];

        match opcode {
            0 => {} // NOP
            1 => self.stack[self.stack_pointer] += 1, // INC [sp]
            2 => self.stack[self.stack_pointer] -= 1, // DEC [sp]
            3 => { // PSH imm
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = imm;
            }
            4 => { // POP
                self.stack[self.stack_pointer] = 0;
                self.stack_pointer -= 1;
            }
            5 => self.stack[self.stack_pointer] += imm, // ADD imm
            6 => self.stack[self.stack_pointer] -= imm,  // SUB imm
            7 => self.stack[self.stack_pointer] *= imm, // MUL imm
            8 => {}, // MEM [mp] = imm 
            _ => {}
        }

        // TODO: add labels
        //self.mem[self.mem_pointer] = opcode; // write the opcode to memory for use later
        self.mem_pointer += 1;
    }
}