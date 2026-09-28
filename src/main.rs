use clap::Parser;
use core::panic;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    fname: String,

    #[arg(short, long)]
    lex: bool,

    #[arg(short, long)]
    parse: bool,

    #[arg(short, long)]
    codegen: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Modes {
    Lex,
    Parse,
    Codegen,
    Full,
}

struct MyParser {
    tokens: Vec<Token>,
    pos: usize,
}

#[derive(Debug)]
struct Program(Function);

#[derive(Debug)]
struct Function(Identifier, Statement);

#[derive(Debug)]
struct Identifier(String);

#[derive(Debug)]
enum Expression {
    Constant(i32),
}

#[derive(Debug)]
enum Statement {
    Return(Expression),
    //If(Expression, Box<Statement>, Option<Box<Statement>>),
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
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
}

fn main() {
    let args = Args::parse();

    let mode;
    if args.codegen {
        mode = Modes::Codegen;
    } else if args.parse {
        mode = Modes::Parse;
    } else if args.lex {
        mode = Modes::Lex;
    } else {
        mode = Modes::Full;
    }

    let f_name = args.fname;
    let contents = fs::read_to_string(f_name).expect("Problem reading file!");

    let tokens = lexer(&contents); // lexing.
    /*
    for tok in &tokens {
        print!("{:?}", tok);
    } 
    */
    let mut parser = MyParser { tokens, pos: 0 };

    if mode > Modes::Lex {
        let res = parser.parse(); // Parse - parse but no generation.
        println!("{:?}", res);
    } else if mode > Modes::Parse {
        todo!(); // Codegen - assembly generation.
    } else {
        todo!(); // emit code.
    }
}

fn lexer(args: &str) -> Vec<Token> {
    // Boundary characeters
    let boundary_characters = [b'(', b')', b'{', b'}', b';', b' ', b'\n'];

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
            } else if cur_char.is_ascii_alphabetic() {
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

    for token in &tokens {
        println!("{:?}", token);
    }

    return tokens;
}

impl MyParser {
    fn parse_tok(&mut self, token: Token) {
        match self.tokens.get(self.pos) {
            Some(cur_tok) => {
                if *cur_tok == token {
                    self.pos += 1
                } else {
                    panic!("Expected {:?}, got {:?}", token, cur_tok)
                }
            }
            None => panic!("Expected {:?}, but got nothing (parsing ended)", token),
        }
    }

    fn parse(&mut self) -> Program {
        let func = self.parse_function();
        return Program(func);
    }

    fn parse_function(&mut self) -> Function {
        self.parse_tok(Token::Int);
        let identifier = self.parse_identifier();
        self.parse_tok(Token::OpenParen);
        self.parse_tok(Token::Void); // Likely to need chaging later.
        self.parse_tok(Token::CloseParen);
        self.parse_tok(Token::OpenCurlyBrace);
        let statement = self.parse_statement();
        self.parse_tok(Token::CloseCurlyBrace);
        return Function(identifier, statement);
    }

    fn parse_identifier(&mut self) -> Identifier {
        let res = match self.tokens.get(self.pos) {
            Some(cur_tok) => {
                if let Token::Identifier(id_name) = cur_tok {
                    Identifier(id_name.clone())
                } else {
                    panic!("Expected an identifier, but got {:?}", cur_tok);
                }
            }
            None => panic!("Expected an identifier, but got nothing (parsing ended)"),
        };
        self.pos += 1;
        return res;
    }

    fn parse_statement(&mut self) -> Statement {
        self.parse_tok(Token::Return);
        let exp = self.parse_expression();
        self.parse_tok(Token::SemiColon);
        return Statement::Return(exp);
    }

    fn parse_expression(&mut self) -> Expression {
        let int_val = self.parse_int();
        return Expression::Constant(int_val);
    }

    fn parse_int(&mut self) -> i32 {
        let res = match self.tokens.get(self.pos) {
            Some(cur_tok) => {
                if let Token::Constant(val) = cur_tok {
                    val
                } else {
                    panic!("Expected an identifier, but got {:?}", cur_tok);
                }
            }
            None => panic!("Expected an identifier, but got nothing (parsing ended)"),
        };
        self.pos += 1;
        return *res;
    }
}
