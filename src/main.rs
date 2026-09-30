use clap::Parser;
use std::fs;

use crate::emit::emit_code;
use crate::lexer::do_lexing;
use crate::parser::MyParser;
use crate::parser::pretty_printer;

mod asts;
mod emit;
mod lexer;
mod parser;

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

    let fname = args.fname;
    let mut out_fname = fname.clone();
    out_fname.truncate(out_fname.len() - 1);
    out_fname = format!("{}s", out_fname);
    let contents = fs::read_to_string(fname).expect("Problem reading file!");

    let tokens = do_lexing(&contents); // lexing.

    let mut parser = MyParser { tokens, pos: 0 };

    match mode {
        Modes::Lex => {}
        Modes::Parse => {
            let parsed = parser.parse();
            pretty_printer(parsed);
        }
        Modes::Codegen => {
            let parsed = parser.parse();
            let ast_assembly = parser::ast_to_ast_a(parsed);
        }
        Modes::Full => {
            let parsed = parser.parse();
            let ast_assembly = parser::ast_to_ast_a(parsed);
            emit_code(ast_assembly, out_fname);
        }
    }
}
