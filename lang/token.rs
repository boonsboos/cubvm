

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TokenKind {
    U, Uprime, U2, // 0, 1, 2
    F, Fprime, F2, // 3, 4, 5
    R, Rprime, R2, // 6, 7 ,8
    B, Bprime, B2, // 9, 10, 11,
    L, Lprime, L2, // 12, 13, 14
    D, Dprime, D2, // 15, 16, 17

    X, Xprime, X2, // 18, 19, 20
    Y, Yprime, Y2, // 21, 22, 23
    Z, Zprime, Z2, // 24, 25, 26
    Asterisk, // 27, noop
    
    // these get transformed, they're just there for the programmer
    M, Mprime, M2,
    S, Sprime, S2,
    E, Eprime, E2,

    // NOTE optionally add wide moves

    Semicolon,
    JumpLabel(String),
    ConditionalLabel(String),
    Comma,

    Newline,
    Unused,
    SOF,
    EOF,
}

pub fn tokenize(lines: Vec<String>) -> Vec<TokenKind> {

    let mut tokens = vec![TokenKind::SOF];

    for line in lines {

        'parts: for token_part in line.split(' ') {

            if token_part.starts_with(":") {
                tokens.push(TokenKind::JumpLabel(token_part.replace(":", "")))
            }
            if token_part.starts_with("=") {
                tokens.push(TokenKind::ConditionalLabel(token_part.replace("=", "")))
            }

            match token_part {
                "U" => tokens.push(TokenKind::U),
                "U'" | "Up" => tokens.push(TokenKind::Uprime),
                "U2" => tokens.push(TokenKind::U2),
                "F" => tokens.push(TokenKind::F),
                "F'" | "Fp" => tokens.push(TokenKind::Fprime),
                "F2" => tokens.push(TokenKind::F2),
                "R" => tokens.push(TokenKind::R),
                "R'" | "Rp" => tokens.push(TokenKind::Rprime),
                "R2" => tokens.push(TokenKind::R2),
                "B" => tokens.push(TokenKind::B),
                "B'" | "Bp" => tokens.push(TokenKind::Bprime),
                "B2" => tokens.push(TokenKind::B2),
                "L" => tokens.push(TokenKind::L),
                "L'" | "Lp" => tokens.push(TokenKind::Lprime),
                "L2" => tokens.push(TokenKind::L2),
                "D" => tokens.push(TokenKind::D),
                "D'" | "Dp" => tokens.push(TokenKind::Dprime),
                "D2" => tokens.push(TokenKind::D2),
                // rotations
                "X" => tokens.push(TokenKind::X),
                "X'" | "Xp" => tokens.push(TokenKind::Xprime),
                "X2" => tokens.push(TokenKind::X2),
                "Y" => tokens.push(TokenKind::Y),
                "Y'" | "Yp" => tokens.push(TokenKind::Yprime),
                "Y2" => tokens.push(TokenKind::Y2),
                "Z" => tokens.push(TokenKind::Z),
                "Z'" | "Zp" => tokens.push(TokenKind::Zprime),
                "Z2" => tokens.push(TokenKind::Z2),
                "*" => tokens.push(TokenKind::Asterisk),
                // slices
                "M" => tokens.push(TokenKind::M),
                "M'" | "Mp" => tokens.push(TokenKind::Mprime),
                "M2" => tokens.push(TokenKind::M2),
                "S" => tokens.push(TokenKind::S),
                "S'" | "Sp" => tokens.push(TokenKind::Sprime),
                "S2" => tokens.push(TokenKind::S2),
                "E" => tokens.push(TokenKind::E),
                "E'" | "Ep" => tokens.push(TokenKind::Eprime),
                "E2" => tokens.push(TokenKind::E2),

                "\n" => tokens.push(TokenKind::Newline),
                ";" => tokens.push(TokenKind::Semicolon),
                "," => tokens.push(TokenKind::Comma),

                "//" => break 'parts, // continue on the next line
                _ => continue
            }
        }

    }
    tokens.push(TokenKind::EOF);

    return tokens;
}

impl TokenKind {
    pub fn opposite(&self) -> TokenKind {
        match self {
            TokenKind::B => TokenKind::Bprime,
            TokenKind::B2 => TokenKind::B2,
            TokenKind::Bprime => TokenKind::B,

            TokenKind::D => TokenKind::Dprime,
            TokenKind::D2 => TokenKind::D2,
            TokenKind::Dprime => TokenKind::D,

            TokenKind::E => TokenKind::Eprime,
            TokenKind::E2 => TokenKind::E2,
            TokenKind::Eprime => TokenKind::E,

            TokenKind::F => TokenKind::Fprime,
            TokenKind::F2 => TokenKind::F2,
            TokenKind::Fprime => TokenKind::F,

            TokenKind::L => TokenKind::Lprime,
            TokenKind::L2 => TokenKind::L2,
            TokenKind::Lprime => TokenKind::L,

            TokenKind::M => TokenKind::Mprime,
            TokenKind::M2 => TokenKind::M2,
            TokenKind::Mprime => TokenKind::M,

            TokenKind::R => TokenKind::Rprime,
            TokenKind::R2 => TokenKind::R2,
            TokenKind::Rprime => TokenKind::R,

            TokenKind::S => TokenKind::Sprime,
            TokenKind::S2 => TokenKind::S2,
            TokenKind::Sprime => TokenKind::S,

            TokenKind::U => TokenKind::Uprime,
            TokenKind::U2 => TokenKind::U2,
            TokenKind::Uprime => TokenKind::U,

            TokenKind::X => TokenKind::Xprime,
            TokenKind::X2 => TokenKind::X2,
            TokenKind::Xprime => TokenKind::X,

            TokenKind::Y => TokenKind::Yprime,
            TokenKind::Y2 => TokenKind::Y2,
            TokenKind::Yprime => TokenKind::Y,

            TokenKind::Z => TokenKind::Zprime,
            TokenKind::Z2 => TokenKind::Z2,
            TokenKind::Zprime => TokenKind::Z,
            _ => TokenKind::Unused,
        }
    }
}