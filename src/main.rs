use clap::Parser;
use std::fs;

use crate::emit::CodeEmitter;
use crate::lexer::do_lexing;
use crate::parser::MyParser;
use crate::parser::pretty_printer::pretty_print;
use crate::semantic_analysis::SemanticAnalysis;
use crate::tacky::TackyTree;

mod assembly;
mod emit;
mod lexer;
mod parser;
mod semantic_analysis;
mod tacky;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    fname: String,

    #[arg(short, long)]
    lex: bool,

    #[arg(short, long)]
    valid: bool,

    #[arg(short, long)]
    tacky: bool,

    #[arg(short, long)]
    parse: bool,

    #[arg(short, long)]
    codegen: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Modes {
    Lex,
    Parse,
    Valid,
    Tacky,
    Codegen,
    Full,
}

fn main() {
    let args = Args::parse();

    let mode;
    if args.codegen {
        mode = Modes::Codegen;
    } else if args.tacky {
        mode = Modes::Tacky;
    } else if args.valid {
        mode = Modes::Valid;
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

    let tokens = do_lexing(&contents); // Lexing is done here!!!

    let mut parser = MyParser { tokens, pos: 0 };

    match mode {
        Modes::Lex => {} // Lexing is done by default above since all modes need it!!!
        Modes::Parse => {
            let parsed = parser.parse();
            pretty_print(&parsed);
        }
        Modes::Valid => {
            let parsed = parser.parse();
            let mut semantic_analysis = SemanticAnalysis {
                ..Default::default()
            };
            let analysed = semantic_analysis.resolve_all(parsed);
            pretty_print(&analysed);
        }
        Modes::Tacky => {
            let parsed = parser.parse();
            let mut semantic_analysis = SemanticAnalysis {
                ..Default::default()
            };
            let analysed = semantic_analysis.resolve_all(parsed);
            pretty_print(&analysed);
            let mut tacky = TackyTree {
                tmp_var: semantic_analysis.tmp_var,
                ..Default::default()
            };
            let tacky_prog = tacky.ast_c_to_tacky(analysed);
            println!("{:?}", tacky_prog);
        }
        Modes::Codegen => {
            let parsed = parser.parse();
            let mut semantic_analysis = SemanticAnalysis {
                ..Default::default()
            };
            let analysed = semantic_analysis.resolve_all(parsed);
            let mut tacky = TackyTree {
                tmp_var: semantic_analysis.tmp_var,
                ..Default::default()
            };
            let tacky_prog = tacky.ast_c_to_tacky(analysed);
            let ast_assembly = assembly::full_conversion_t_to_a(tacky_prog);
            println!("{:?}", ast_assembly);
        }
        Modes::Full => {
            let parsed = parser.parse();
            let mut semantic_analysis = SemanticAnalysis {
                ..Default::default()
            };
            let analysed = semantic_analysis.resolve_all(parsed);
            let mut tacky = TackyTree {
                tmp_var: semantic_analysis.tmp_var,
                ..Default::default()
            };
            let tacky_prog = tacky.ast_c_to_tacky(analysed);
            let ast_assembly = assembly::full_conversion_t_to_a(tacky_prog);
            let code_emitter = CodeEmitter {
                program: ast_assembly,
                fname: out_fname,
            };
            code_emitter.emit_code();
        }
    }
}
