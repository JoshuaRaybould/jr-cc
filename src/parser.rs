use crate::ast_c::*;
use crate::lexer::Token;

pub struct MyParser {
    pub tokens: Vec<Token>,
    pub pos: usize,
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

    pub fn parse(&mut self) -> Program {
        let func = self.parse_function();
        if self.pos < self.tokens.len() {
            panic!(
                "Not all tokens consumed, currently at {:?}",
                self.tokens.get(self.pos)
            );
        }
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

fn get_indentation(indent: usize) -> String {
    " ".repeat(4).repeat(indent)
}

pub fn pretty_printer(program: Program) {
    let indent = 0;
    let Program(func) = program;

    println!("Program(");
    pretty_print_function(func, indent + 1);
    println!(")")
}

fn pretty_print_function(func: Function, indent: usize) {
    let indentation = get_indentation(indent);
    let Function(identifier, statement) = func;

    println!("{}Function(", indentation);
    pretty_print_identifier(identifier, indent + 1);
    pretty_print_statement(statement, indent + 1);
    println!("{})", indentation);
}

fn pretty_print_identifier(identifier: Identifier, indent: usize) {
    let indentation = get_indentation(indent);
    let Identifier(id_string) = identifier;
    println!("{}name=\"{}\",", indentation, id_string);
}

fn pretty_print_statement(statement: Statement, indent: usize) {
    let indentation = get_indentation(indent);

    match statement {
        Statement::Return(expression) => {
            println!("{}body=Return(", indentation);
            pretty_print_expression(expression, indent + 1);
            println!("{})", indentation);
        }
    }
}

fn pretty_print_expression(expression: Expression, indent: usize) {
    let indentation = get_indentation(indent);

    match expression {
        Expression::Constant(val) => println!("{}Constant({})", indentation, val),
    }
}
