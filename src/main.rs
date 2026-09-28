use clap::Parser;
use std::fs;

use crate::lexer::do_lexing;
use crate::parser::MyParser;
use crate::parser::pretty_printer;

mod asts;
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

    let f_name = args.fname;
    let contents = fs::read_to_string(f_name).expect("Problem reading file!");

    let tokens = do_lexing(&contents); // lexing.

    let mut parser = MyParser { tokens, pos: 0 };

    let parsed;
    let ast_assembly;
    if mode > Modes::Lex {
        parsed = parser.parse(); // Parse - parse but no generation.
        pretty_printer(parsed);
    } else if mode > Modes::Parse {
        parsed = parser.parse();
        ast_assembly = parser::ast_to_ast_a(parsed); // Codegen - assembly generation.
    } else if mode > Modes::Codegen {
        todo!(); // emit code.
    }
}
