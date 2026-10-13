#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Identifier(String),
    Constant(i32),
    Int,
    Void,
    Return,
    OpenParen,
    CloseParen,
    OpenCurlyBrace,
    CloseCurlyBrace,
    SemiColon,
    Complement,
    Negate,
    Decrement,
    Add,
    Multiply,
    Divide,
    Remainder,
    BitAnd,
    BitOr,
    Xor,
    LeftShift,
    RightShift,
    And,
    Or,
    Not,
    Equal,
    Assign,
    Neq,
    Le,
    Leq,
    Ge,
    Geq,
}

impl Token {
    pub fn get_precedence(&self) -> i32 {
        const SECOND_HIGHEST: i32 = 70;
        const BIN_MULT_DIV: i32 = 60;
        const BIN_PLUS_MINUS: i32 = 50;
        const BIT_SHIFT: i32 = 45;
        const COMPARING: i32 = 42;
        const EQUIVALENCE: i32 = 40;
        const BIT_AND: i32 = 36;
        const BIT_XOR: i32 = 35;
        const BIT_OR: i32 = 34;
        const LOGIC_AND: i32 = 30;
        const LOGIC_OR: i32 = 28;
        const ASSIGNMENT: i32 = 20;
        match self {
            Token::Not => SECOND_HIGHEST,
            Token::Remainder => BIN_MULT_DIV,
            Token::Multiply => BIN_MULT_DIV,
            Token::Divide => BIN_MULT_DIV,
            Token::Negate => BIN_PLUS_MINUS,
            Token::Add => BIN_PLUS_MINUS,
            Token::LeftShift => BIT_SHIFT,
            Token::RightShift => BIT_SHIFT,
            Token::Ge => COMPARING,
            Token::Geq => COMPARING,
            Token::Le => COMPARING,
            Token::Leq => COMPARING,
            Token::Equal => EQUIVALENCE,
            Token::Neq => EQUIVALENCE,
            Token::BitAnd => BIT_AND,
            Token::Xor => BIT_XOR,
            Token::BitOr => BIT_OR,
            Token::And => LOGIC_AND,
            Token::Or => LOGIC_OR,
            Token::Assign => ASSIGNMENT,
            _ => panic!("Incorrect usage"),
        }
    }
}

pub fn do_lexing(args: &str) -> Vec<Token> {
    // Boundary characeters
    let boundary_characters = [
        b'(', b')', b'{', b'}', b';', b'-', b'~', b'+', b'*', b'/', b'%', b'&', b'|', b'^', b'<',
        b'>', b'!', b'=', b' ', b'\n',
    ];

    // Identifiers
    const INT_KEYWORD: &[u8] = "int".as_bytes();
    const VOID_KEYWORD: &[u8] = "void".as_bytes();
    const RETURN_KEYWORD: &[u8] = "return".as_bytes();

    let mut tokens = Vec::new();
    let byte_str = args.as_bytes();
    let mut l = 0;
    let mut r;

    while l < byte_str.len() {
        let cur_char = byte_str[l];
        if cur_char == b' ' || cur_char == b'\n' {
            // Trim white space.
            l += 1;
        } else {
            if boundary_characters.contains(&cur_char) {
                // Boundary characetrs
                let cur_token = match &cur_char {
                    b'(' => Token::OpenParen,
                    b')' => Token::CloseParen,
                    b'{' => Token::OpenCurlyBrace,
                    b'}' => Token::CloseCurlyBrace,
                    b';' => Token::SemiColon,
                    b'~' => Token::Complement,
                    b'-' => {
                        if l + 1 < byte_str.len() && byte_str[l + 1] == b'-' {
                            l += 1;
                            Token::Decrement
                        } else {
                            Token::Negate
                        }
                    }
                    b'+' => Token::Add,
                    b'/' => Token::Divide,
                    b'*' => Token::Multiply,
                    b'%' => Token::Remainder,
                    b'&' => {
                        if l + 1 < byte_str.len() && byte_str[l + 1] == b'&' {
                            l += 1;
                            Token::And
                        } else {
                            Token::BitAnd
                        }
                    }
                    b'|' => {
                        if l + 1 < byte_str.len() && byte_str[l + 1] == b'|' {
                            l += 1;
                            Token::Or
                        } else {
                            Token::BitOr
                        }
                    }
                    b'^' => Token::Xor,
                    b'<' => {
                        if l + 1 < byte_str.len() && byte_str[l + 1] == b'<' {
                            l += 1;
                            Token::LeftShift
                        } else if byte_str[l + 1] == b'=' {
                            l += 1;
                            Token::Leq
                        } else {
                            Token::Le
                        }
                    }
                    b'>' => {
                        if l + 1 < byte_str.len() && byte_str[l + 1] == b'>' {
                            l += 1;
                            Token::RightShift
                        } else if l + 1 < byte_str.len() && byte_str[l + 1] == b'=' {
                            l += 1;
                            Token::Geq
                        } else {
                            Token::Ge
                        }
                    }
                    b'!' => {
                        if l + 1 < byte_str.len() && byte_str[l + 1] == b'=' {
                            l += 1;
                            Token::Neq
                        } else {
                            Token::Not
                        }
                    }
                    b'=' => {
                        if l + 1 < byte_str.len() && byte_str[l + 1] == b'=' {
                            l += 1;
                            Token::Equal
                        } else {
                            Token::Assign
                        }
                    }
                    _ => panic!("How"),
                };
                tokens.push(cur_token);
                l += 1
            } else if cur_char.is_ascii_digit() {
                // Integer case.
                r = l + 1;

                while r < byte_str.len() && byte_str[r].is_ascii_digit() {
                    r += 1;
                }
                if r >= byte_str.len() || !boundary_characters.contains(&byte_str[r]) {
                    panic!("An integer starts but has no word boundary character at the end");
                } else {
                    let tok_str = std::str::from_utf8(&byte_str[l..r]).unwrap();
                    let cur_token = tok_str.parse().unwrap();
                    tokens.push(Token::Constant(cur_token));
                }
                l = r;
            } else if cur_char.is_ascii_alphabetic() || cur_char == b'_' {
                // Identifier case
                r = l + 1;

                while r < byte_str.len()
                    && (byte_str[r].is_ascii_alphanumeric() || byte_str[r] == b'_')
                {
                    r += 1;
                }
                if r >= byte_str.len() || !boundary_characters.contains(&byte_str[r]) {
                    panic!("An integer starts but has no word boundary character at the end");
                } else {
                    let cur_token = match &byte_str[l..r] {
                        INT_KEYWORD => Token::Int,
                        VOID_KEYWORD => Token::Void,
                        RETURN_KEYWORD => Token::Return,
                        identifier => {
                            Token::Identifier(String::from_utf8(identifier.to_vec()).expect(""))
                        }
                    };
                    tokens.push(cur_token);
                }
                l = r;
            } else {
                panic!("Token starting with {}", char::from(cur_char));
            }
        }
    }

    return tokens;
}
