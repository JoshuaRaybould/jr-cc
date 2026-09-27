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
/* 
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Modes {
    Lex,
    Parse,
    Codegen,
    Full,
}*/

#[derive(Debug)]
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
/* 
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
*/
    let f_name = args.fname;
    let contents = fs::read_to_string(f_name).expect("Problem reading file!");

    let _res = lexer(&contents); // lexing.
    /* 
    if mode > Modes::Lex {
        todo!(); // Parse - parse but no generation.
    } else if mode > Modes::Parse {
        todo!(); // Codegen - assembly generation.
    } else {
        todo!(); // emit code.
    }
    */
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
                if r >= byte_str.len() || !boundary_characters.contains(&byte_str[r]){
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

                while r < byte_str.len() && (byte_str[r].is_ascii_alphanumeric() || byte_str[r] == b'_') {
                    r += 1;
                }
                if r >= byte_str.len() || !boundary_characters.contains(&byte_str[r]){
                    panic!("An integer starts but has no word boundary character at the end");
                } else {
                    let cur_token = match &byte_str[l..r] {
                        INT_KEYWORD => Token::Int,
                        VOID_KEYWORD => Token::Void,
                        RETURN_KEYWORD => Token::Return,
                        identifier => Token::Identifier(String::from_utf8(identifier.to_vec()).expect("")),
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
