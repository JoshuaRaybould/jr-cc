pub mod ast;
pub mod pretty_printer;

use crate::lexer::Token;
use crate::parser::ast::{
    BinaryOperator as CBinaryOperator, Expression as CExpression, Function as CFunction,
    Identifier as CIdentifier, Program as CProgram, Statement as CStatement,
    UnaryOperator as CUnaryOperator,
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
        let exp = self.parse_expression(0);
        self.parse_tok(Token::SemiColon);
        return CStatement::Return(exp);
    }

    fn parse_expression(&mut self, min_prec: i32) -> CExpression {
        let mut left = self.parse_factor();
        let mut next_tok = self
            .tokens
            .get(self.pos)
            .expect("Program ended with an expression");

        let binary_operators = [
            Token::Add,
            Token::Negate,
            Token::Divide,
            Token::Multiply,
            Token::Remainder,
            Token::BitAnd,
            Token::BitOr,
            Token::Xor,
            Token::LeftShift,
            Token::RightShift,
            Token::And,
            Token::Or,
            Token::Equal,
            Token::Neq,
            Token::Le,
            Token::Leq,
            Token::Ge,
            Token::Geq,
            Token::Assign,
        ];

        while binary_operators.contains(next_tok) && next_tok.get_precedence() >= min_prec {
            let op = match next_tok {
                Token::Add => CBinaryOperator::Add,
                Token::Negate => CBinaryOperator::Subtract,
                Token::Divide => CBinaryOperator::Divide,
                Token::Multiply => CBinaryOperator::Multiply,
                Token::Remainder => CBinaryOperator::Remainder,
                Token::BitAnd => CBinaryOperator::BitAnd,
                Token::BitOr => CBinaryOperator::BitOr,
                Token::Xor => CBinaryOperator::Xor,
                Token::LeftShift => CBinaryOperator::LeftShift,
                Token::RightShift => CBinaryOperator::RightShift,
                Token::And => CBinaryOperator::And,
                Token::Or => CBinaryOperator::Or,
                Token::Equal => CBinaryOperator::Equal,
                Token::Neq => CBinaryOperator::Neq,
                Token::Le => CBinaryOperator::Le,
                Token::Leq => CBinaryOperator::Leq,
                Token::Ge => CBinaryOperator::Ge,
                Token::Geq => CBinaryOperator::Geq,
                Token::Assign => CBinaryOperator::Assign,
                _ => panic!("Impossible!"),
            };
            let precedence = next_tok.get_precedence();
            self.parse_tok(next_tok.clone());
            let right = self.parse_expression(precedence + 1);
            left = CExpression::Binary(op, Box::new(left), Box::new(right));
            next_tok = self
                .tokens
                .get(self.pos)
                .expect("Program ended with an expression");
        }

        return left;
    }

    fn parse_factor(&mut self) -> CExpression {
        let first_tok = self
            .tokens
            .get(self.pos)
            .expect("Expected an expression but got nothing");

        let exp = match first_tok {
            Token::OpenParen => {
                self.parse_tok(Token::OpenParen);
                let inner_exp = self.parse_expression(0);
                self.parse_tok(Token::CloseParen);
                inner_exp
            }
            Token::Negate => {
                self.parse_tok(Token::Negate);
                let inner_exp = self.parse_factor();
                CExpression::Unary(CUnaryOperator::Negate, Box::new(inner_exp))
            }
            Token::Complement => {
                self.parse_tok(Token::Complement);
                let inner_exp = self.parse_factor();
                CExpression::Unary(CUnaryOperator::Complement, Box::new(inner_exp))
            }
            Token::Not => {
                self.parse_tok(Token::Not);
                let inner_exp = self.parse_factor();
                CExpression::Unary(CUnaryOperator::Not, Box::new(inner_exp))
            }
            Token::Constant(_) => CExpression::Constant(self.parse_int()),
            x => panic!("Malformed expression! Got {:?}", x),
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
