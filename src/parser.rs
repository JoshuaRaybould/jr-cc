pub mod ast;
pub mod pretty_printer;

use crate::lexer::Token;
use crate::parser::ast::{
    Expression as CExpression, Function as CFunction, Identifier as CIdentifier,
    Program as CProgram, Statement as CStatement, UnaryOperator as CUnaryOperator,
};

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

    pub fn parse(&mut self) -> CProgram {
        let func = self.parse_function();
        if self.pos < self.tokens.len() {
            panic!(
                "Not all tokens consumed, currently at {:?}",
                self.tokens.get(self.pos)
            );
        }
        return CProgram(func);
    }

    fn parse_function(&mut self) -> CFunction {
        self.parse_tok(Token::Int);
        let identifier = self.parse_identifier();
        self.parse_tok(Token::OpenParen);
        self.parse_tok(Token::Void); // Likely to need chaging later.
        self.parse_tok(Token::CloseParen);
        self.parse_tok(Token::OpenCurlyBrace);
        let statement = self.parse_statement();
        self.parse_tok(Token::CloseCurlyBrace);
        return CFunction(identifier, statement);
    }

    fn parse_identifier(&mut self) -> CIdentifier {
        let res = match self.tokens.get(self.pos) {
            Some(cur_tok) => {
                if let Token::Identifier(id_name) = cur_tok {
                    CIdentifier(id_name.clone())
                } else {
                    panic!("Expected an identifier, but got {:?}", cur_tok);
                }
            }
            None => panic!("Expected an identifier, but got nothing (parsing ended)"),
        };
        self.pos += 1;
        return res;
    }

    fn parse_statement(&mut self) -> CStatement {
        self.parse_tok(Token::Return);
        let exp = self.parse_expression();
        self.parse_tok(Token::SemiColon);
        return CStatement::Return(exp);
    }

    fn parse_expression(&mut self) -> CExpression {
        let first_tok = self
            .tokens
            .get(self.pos)
            .expect("Expected and expression but got nothing");

        let exp = match first_tok {
            Token::OpenParen => {
                self.parse_tok(Token::OpenParen);
                let inner_exp = self.parse_expression();
                self.parse_tok(Token::CloseParen);
                inner_exp
            }
            Token::Negate => {
                self.parse_tok(Token::Negate);
                CExpression::Unary(CUnaryOperator::Negate, Box::new(self.parse_expression()))
            }
            Token::Complement => {
                self.parse_tok(Token::Complement);
                CExpression::Unary(
                    CUnaryOperator::Complement,
                    Box::new(self.parse_expression()),
                )
            }
            Token::Constant(_) => CExpression::Constant(self.parse_int()),
            _ => panic!("Malformed expression!"),
        };

        return exp;
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
