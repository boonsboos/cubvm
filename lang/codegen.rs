use std::{collections::HashMap, fmt::Error, process};

use super::token::TokenKind;

#[derive(Debug)]
pub struct Code {
    pub movesets: Vec<u16>,
}

pub fn generate(tokens: Vec<TokenKind>) -> Result<Code, Error> {

    let mut line_count: usize = 1;

    let mut label_map: HashMap<TokenKind, u16> = HashMap::new();

    let mut buf: Vec<u16> = vec![0xB0u16]; // SOF written above
    let mut i: usize = 1;
    while i < tokens.len()-1 { // saves reading past EOF
        if tokens[i] == TokenKind::Newline {
            line_count += 1;
        }

        // simple optimization by skipping moves that cancel each other out
        if tokens[i+1] != tokens[i].opposite() && tokens[i].opposite() != tokens[i-1] {
            match tokens[i] {
                // check token.rs
                TokenKind::U => buf.push(0),
                TokenKind::Uprime => buf.push(1),
                TokenKind::U2 => buf.push(2),
                TokenKind::F => buf.push(3),
                TokenKind::Fprime => buf.push(4),
                TokenKind::F2 => buf.push(5),
                TokenKind::R => buf.push(6),
                TokenKind::Rprime => buf.push(7),
                TokenKind::R2 => buf.push(8),
                TokenKind::B => buf.push(9),
                TokenKind::Bprime => buf.push(10),
                TokenKind::B2 => buf.push(11),
                TokenKind::L => buf.push(12),
                TokenKind::Lprime => buf.push(13),
                TokenKind::L2 => buf.push(14),
                TokenKind::D => buf.push(15),
                TokenKind::Dprime => buf.push(16),
                TokenKind::D2 => buf.push(17),

                TokenKind::X => buf.push(18),
                TokenKind::Xprime => buf.push(19),
                TokenKind::X2 => buf.push(20),
                TokenKind::Y => buf.push(21),
                TokenKind::Yprime => buf.push(22),
                TokenKind::Y2 => buf.push(23),
                TokenKind::Z => buf.push(24),
                TokenKind::Zprime => buf.push(25),
                TokenKind::Z2 => buf.push(26),

                TokenKind::Asterisk => buf.push(27),

                // slice moves are sort of macros
                TokenKind::M => {
                    buf.push(13); // L'
                    buf.push(6);  // R
                    buf.push(18); // X
                }
                TokenKind::Mprime => {
                    buf.push(12); // L
                    buf.push(7);  // R'
                    buf.push(19); // X'
                }
                TokenKind::M2 => {
                    buf.push(14); // L2
                    buf.push(8);  // R2
                    buf.push(20); // X2
                }
                TokenKind::S => {
                    buf.push(3);  // F
                    buf.push(10); // B'
                    buf.push(25); // Z'
                }
                TokenKind::Sprime => {
                    buf.push(4);  // F'
                    buf.push(9);  // B
                    buf.push(24); // Z
                }
                TokenKind::S2 => {
                    buf.push(5);  // F2
                    buf.push(11); // B2
                    buf.push(26); // Z2
                }
                TokenKind::E => {
                    buf.push(0);  // U
                    buf.push(16); // D'
                    buf.push(22); // Y'
                }
                TokenKind::Eprime => {
                    buf.push(1);  // U'
                    buf.push(15); // D
                    buf.push(21); // Y
                }
                TokenKind::E2 => {
                    buf.push(2);  // U2
                    buf.push(17); // D2
                    buf.push(23); // Y2
                }

                TokenKind::Semicolon => buf.push(b';'.into()),
                TokenKind::Comma => buf.push(b','.into()),

                TokenKind::JumpLabel(_) => {

                    if label_map.contains_key(&tokens[i]) {
                        buf.push(0x003Au16); // insert an ascii : to signal a jump label
                        buf.push(*label_map.get(&tokens[i]).unwrap())
                    } else {
                        // check the unknown label isn't defined in the middle of the line
                        if ![TokenKind::SOF, TokenKind::Newline].contains(&tokens[i-1]) {
                            println!("error on line {}: Labels can only be defined at the start of a new line", line_count);
                            process::exit(1);
                        }

                        label_map.insert(tokens[i].clone(), buf.len().try_into().unwrap());
                        //buf.push(buf.len().try_into().unwrap()) // always point to the next instruction
                    }
                }
                TokenKind::ConditionalLabel(_) => {

                    if label_map.contains_key(&tokens[i]) {
                        buf.push(0x003Du16); // insert an ascii = to signal a conditional label
                        buf.push(*label_map.get(&tokens[i]).unwrap())
                    } else {
                        // check the unknown label isn't defined in the middle of the line
                        if ![TokenKind::SOF, TokenKind::Newline].contains(&tokens[i-1]) {
                            println!("error on line {}: Labels can only be defined at the start of a new line", line_count);
                            process::exit(1);
                        }

                        label_map.insert(tokens[i].clone(), buf.len().try_into().unwrap());
                        //buf.push(buf.len().try_into().unwrap()) // always point to the next instruction
                    }
                }
                TokenKind::EOF => break, // should actually panic but this works too
                _ => {},
            }
        }

        i += 1; 
    }

    Ok(Code{movesets: buf})
}