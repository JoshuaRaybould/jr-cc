#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub position: Position,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Position {
    line: u32,
    column: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
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

impl TokenType {
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
            TokenType::Not => SECOND_HIGHEST,
            TokenType::Remainder => BIN_MULT_DIV,
            TokenType::Multiply => BIN_MULT_DIV,
            TokenType::Divide => BIN_MULT_DIV,
            TokenType::Negate => BIN_PLUS_MINUS,
            TokenType::Add => BIN_PLUS_MINUS,
            TokenType::LeftShift => BIT_SHIFT,
            TokenType::RightShift => BIT_SHIFT,
            TokenType::Ge => COMPARING,
            TokenType::Geq => COMPARING,
            TokenType::Le => COMPARING,
            TokenType::Leq => COMPARING,
            TokenType::Equal => EQUIVALENCE,
            TokenType::Neq => EQUIVALENCE,
            TokenType::BitAnd => BIT_AND,
            TokenType::Xor => BIT_XOR,
            TokenType::BitOr => BIT_OR,
            TokenType::And => LOGIC_AND,
            TokenType::Or => LOGIC_OR,
            TokenType::Assign => ASSIGNMENT,
            _ => panic!("Incorrect usage"),
        }
    }
}

pub struct Lexer {
    pub line: u32,
    pub column: u32,
}

impl Lexer {
    pub fn do_lexing(&mut self, args: &str) -> Vec<Token> {
        // Boundary characeters
        let boundary_characters = [
            b'(', b')', b'{', b'}', b';', b'-', b'~', b'+', b'*', b'/', b'%', b'&', b'|', b'^',
            b'<', b'>', b'!', b'=', b' ', b'\n',
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
            if cur_char == b' ' {
                // Trim white space.
                l += 1;
                self.column += 1;
            } else if cur_char == b'\n' {
                // Move to next line
                l += 1;
                self.line += 1;
                self.column = 0;
            } else {
                if boundary_characters.contains(&cur_char) {
                    // Boundary characetrs
                    let tok_type = match &cur_char {
                        b'(' => TokenType::OpenParen,
                        b')' => TokenType::CloseParen,
                        b'{' => TokenType::OpenCurlyBrace,
                        b'}' => TokenType::CloseCurlyBrace,
                        b';' => TokenType::SemiColon,
                        b'~' => TokenType::Complement,
                        b'-' => {
                            if l + 1 < byte_str.len() && byte_str[l + 1] == b'-' {
                                l += 1;
                                self.column += 1;
                                TokenType::Decrement
                            } else {
                                TokenType::Negate
                            }
                        }
                        b'+' => TokenType::Add,
                        b'/' => TokenType::Divide,
                        b'*' => TokenType::Multiply,
                        b'%' => TokenType::Remainder,
                        b'&' => {
                            if l + 1 < byte_str.len() && byte_str[l + 1] == b'&' {
                                l += 1;
                                self.column += 1;
                                TokenType::And
                            } else {
                                TokenType::BitAnd
                            }
                        }
                        b'|' => {
                            if l + 1 < byte_str.len() && byte_str[l + 1] == b'|' {
                                l += 1;
                                self.column += 1;
                                TokenType::Or
                            } else {
                                TokenType::BitOr
                            }
                        }
                        b'^' => TokenType::Xor,
                        b'<' => {
                            if l + 1 < byte_str.len() && byte_str[l + 1] == b'<' {
                                l += 1;
                                self.column += 1;
                                TokenType::LeftShift
                            } else if byte_str[l + 1] == b'=' {
                                l += 1;
                                TokenType::Leq
                            } else {
                                TokenType::Le
                            }
                        }
                        b'>' => {
                            if l + 1 < byte_str.len() && byte_str[l + 1] == b'>' {
                                l += 1;
                                self.column += 1;
                                TokenType::RightShift
                            } else if l + 1 < byte_str.len() && byte_str[l + 1] == b'=' {
                                l += 1;
                                self.column += 1;
                                TokenType::Geq
                            } else {
                                TokenType::Ge
                            }
                        }
                        b'!' => {
                            if l + 1 < byte_str.len() && byte_str[l + 1] == b'=' {
                                l += 1;
                                self.column += 1;
                                TokenType::Neq
                            } else {
                                TokenType::Not
                            }
                        }
                        b'=' => {
                            if l + 1 < byte_str.len() && byte_str[l + 1] == b'=' {
                                l += 1;
                                self.column += 1;
                                TokenType::Equal
                            } else {
                                TokenType::Assign
                            }
                        }
                        _ => panic!("Unreachable!"),
                    };
                    let position = Position {
                        line: self.line,
                        column: self.column,
                    };

                    let cur_token = Token {
                        token_type: tok_type,
                        position,
                    };
                    tokens.push(cur_token);
                    l += 1;
                    self.column += 1;
                } else if cur_char.is_ascii_digit() {
                    // Integer case.
                    r = l + 1;

                    while r < byte_str.len() && byte_str[r].is_ascii_digit() {
                        r += 1;
                        self.column += 1;
                    }
                    if r >= byte_str.len() || !boundary_characters.contains(&byte_str[r]) {
                        panic!("An integer starts but has no word boundary character at the end");
                    } else {
                        let tok_str = std::str::from_utf8(&byte_str[l..r]).unwrap();
                        let tok_int = tok_str.parse().unwrap();
                        let tok_type = TokenType::Constant(tok_int);
                        let position = Position {
                            line: self.line,
                            column: self.column,
                        };
                        let cur_token = Token {
                            token_type: tok_type,
                            position,
                        };
                        tokens.push(cur_token);
                        self.column += 1;
                    }
                    l = r;
                } else if cur_char.is_ascii_alphabetic() || cur_char == b'_' {
                    // Identifier case
                    r = l + 1;

                    while r < byte_str.len()
                        && (byte_str[r].is_ascii_alphanumeric() || byte_str[r] == b'_')
                    {
                        r += 1;
                        self.column += 1;
                    }
                    if r >= byte_str.len() || !boundary_characters.contains(&byte_str[r]) {
                        panic!("An integer starts but has no word boundary character at the end");
                    } else {
                        let tok_type = match &byte_str[l..r] {
                            INT_KEYWORD => TokenType::Int,
                            VOID_KEYWORD => TokenType::Void,
                            RETURN_KEYWORD => TokenType::Return,
                            identifier => TokenType::Identifier(
                                String::from_utf8(identifier.to_vec()).expect(""),
                            ),
                        };
                        let position = Position {
                            line: self.line,
                            column: self.column,
                        };
                        let cur_token = Token {
                            token_type: tok_type,
                            position,
                        };
                        tokens.push(cur_token);
                        self.column += 1;
                    }
                    l = r;
                } else {
                    panic!("Token starting with {}", char::from(cur_char));
                }
            }
        }

        return tokens;
    }
}
