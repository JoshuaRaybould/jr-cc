pub mod ast;
pub mod pretty_printer;

use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::parser::ast::{
    BinaryOperator as CBinaryOperator, BlockItem as CBlockItem, Declaration as CDeclaration,
    Expression as CExpression, Function as CFunction, Identifier as CIdentifier,
    Program as CProgram, Statement as CStatement, UnaryOperator as CUnaryOperator,
};

pub struct MyParser {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

impl MyParser {
    fn parse_tok(&mut self, token: TokenType) {
        match self.tokens.get(self.pos) {
            Some(cur_tok) => {
                if cur_tok.token_type == token {
                    self.pos += 1
                } else {
                    println!("{:?}", cur_tok.position);
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
        self.parse_tok(TokenType::Int);
        let identifier = self.parse_identifier();
        self.parse_tok(TokenType::OpenParen);
        self.parse_tok(TokenType::Void); // Likely to need chaging later.
        self.parse_tok(TokenType::CloseParen);
        self.parse_tok(TokenType::OpenCurlyBrace);
        let block_items = self.parse_body();
        self.parse_tok(TokenType::CloseCurlyBrace);
        return CFunction(identifier, block_items);
    }

    fn get_token(&self, expected: &str) -> &Token {
        match self.tokens.get(self.pos) {
            Some(cur_tok) => return &cur_tok,
            None => panic!(
                "Expected an {:?}, but got nothing (parsing ended)",
                expected
            ),
        };
    }

    fn parse_identifier(&mut self) -> CIdentifier {
        let expected_tok_name = "identifier";
        let cur_tok = self.get_token(expected_tok_name);
        let res;
        if let TokenType::Identifier(id_name) = &cur_tok.token_type {
            res = CIdentifier(id_name.clone())
        } else {
            self.got_unexpected_token(expected_tok_name, cur_tok)
        }

        self.pos += 1;
        return res;
    }

    fn parse_body(&mut self) -> Vec<CBlockItem> {
        let mut block_items = Vec::new();
        loop {
            let cur_tok = self.get_token("statement or declaration");
            if cur_tok.token_type == TokenType::CloseCurlyBrace {
                return block_items;
            }
            let block_item = self.parse_block_item();
            block_items.push(block_item);
        }
    }

    fn parse_block_item(&mut self) -> CBlockItem {
        let cur_tok = self.get_token("statement or declaration");
        if cur_tok.token_type == TokenType::Int {
            let declaration = self.parse_declaration();
            return CBlockItem::D(declaration);
        } else {
            let statement = self.parse_statement();
            return CBlockItem::S(statement);
        }
    }

    fn parse_declaration(&mut self) -> CDeclaration {
        self.parse_tok(TokenType::Int);
        let identifier = self.parse_identifier();

        let cur_tok = self.get_token("= (assignment) or ;");
        if cur_tok.token_type == TokenType::Assign {
            self.parse_tok(TokenType::Assign);
            let exp = self.parse_expression(0);
            self.parse_tok(TokenType::SemiColon);
            return CDeclaration(identifier, Some(exp));
        }
        self.parse_tok(TokenType::SemiColon);
        return CDeclaration(identifier, None);
    }

    fn parse_statement(&mut self) -> CStatement {
        let cur_tok = self.get_token("return, ; or an expression");
        if cur_tok.token_type == TokenType::Return {
            self.parse_tok(TokenType::Return);
            let exp = self.parse_expression(0);
            self.parse_tok(TokenType::SemiColon);
            return CStatement::Return(exp);
        } else if cur_tok.token_type == TokenType::SemiColon {
            self.parse_tok(TokenType::SemiColon);
            return CStatement::Null;
        } else {
            let exp = self.parse_expression(0);
            self.parse_tok(TokenType::SemiColon);
            return CStatement::Exp(exp);
        }
    }

    fn parse_expression(&mut self, min_prec: i32) -> CExpression {
        let mut left = self.parse_factor();
        let mut next_tok = self
            .tokens
            .get(self.pos)
            .expect("Program ended with an expression");
        let mut next_tok_type = next_tok.token_type.clone();

        let binary_operators = [
            TokenType::Add,
            TokenType::Negate,
            TokenType::Divide,
            TokenType::Multiply,
            TokenType::Remainder,
            TokenType::BitAnd,
            TokenType::BitOr,
            TokenType::Xor,
            TokenType::LeftShift,
            TokenType::RightShift,
            TokenType::And,
            TokenType::Or,
            TokenType::Equal,
            TokenType::Neq,
            TokenType::Le,
            TokenType::Leq,
            TokenType::Ge,
            TokenType::Geq,
            TokenType::Assign,
        ];

        while binary_operators.contains(&next_tok_type)
            && next_tok_type.get_precedence() >= min_prec
        {
            let op = match next_tok_type {
                TokenType::Add => CBinaryOperator::Add,
                TokenType::Negate => CBinaryOperator::Subtract,
                TokenType::Divide => CBinaryOperator::Divide,
                TokenType::Multiply => CBinaryOperator::Multiply,
                TokenType::Remainder => CBinaryOperator::Remainder,
                TokenType::BitAnd => CBinaryOperator::BitAnd,
                TokenType::BitOr => CBinaryOperator::BitOr,
                TokenType::Xor => CBinaryOperator::Xor,
                TokenType::LeftShift => CBinaryOperator::LeftShift,
                TokenType::RightShift => CBinaryOperator::RightShift,
                TokenType::And => CBinaryOperator::And,
                TokenType::Or => CBinaryOperator::Or,
                TokenType::Equal => CBinaryOperator::Equal,
                TokenType::Neq => CBinaryOperator::Neq,
                TokenType::Le => CBinaryOperator::Le,
                TokenType::Leq => CBinaryOperator::Leq,
                TokenType::Ge => CBinaryOperator::Ge,
                TokenType::Geq => CBinaryOperator::Geq,
                TokenType::Assign => CBinaryOperator::Assign,
                _ => panic!("Impossible!"),
            };

            if op == CBinaryOperator::Assign {
                let precedence = next_tok_type.get_precedence();
                self.parse_tok(TokenType::Assign);
                let right = self.parse_expression(precedence);
                left = CExpression::Assignment(Box::new(left), Box::new(right));
            } else {
                let precedence = next_tok_type.get_precedence();
                self.parse_tok(next_tok_type.clone());
                let right = self.parse_expression(precedence + 1);
                left = CExpression::Binary(op, Box::new(left), Box::new(right));
            }
            next_tok = self
                .tokens
                .get(self.pos)
                .expect("Program ended with an expression");
            next_tok_type = next_tok.token_type.clone();
        }

        return left;
    }

    fn parse_factor(&mut self) -> CExpression {
        let first_tok = self
            .tokens
            .get(self.pos)
            .expect("Expected an expression but got nothing");

        let exp = match &first_tok.token_type {
            TokenType::OpenParen => {
                self.parse_tok(TokenType::OpenParen);
                let inner_exp = self.parse_expression(0);
                self.parse_tok(TokenType::CloseParen);
                inner_exp
            }
            TokenType::Negate => {
                self.parse_tok(TokenType::Negate);
                let inner_exp = self.parse_factor();
                CExpression::Unary(CUnaryOperator::Negate, Box::new(inner_exp))
            }
            TokenType::Complement => {
                self.parse_tok(TokenType::Complement);
                let inner_exp = self.parse_factor();
                CExpression::Unary(CUnaryOperator::Complement, Box::new(inner_exp))
            }
            TokenType::Not => {
                self.parse_tok(TokenType::Not);
                let inner_exp = self.parse_factor();
                CExpression::Unary(CUnaryOperator::Not, Box::new(inner_exp))
            }
            TokenType::Constant(_) => CExpression::Constant(self.parse_int()),
            TokenType::Identifier(_) => CExpression::Var(self.parse_var()),
            x => panic!("Malformed expression! Got {:?}", x),
        };

        return exp;
    }

    fn parse_int(&mut self) -> i32 {
        let expected_tok_name = "int";
        let cur_tok = self.get_token(expected_tok_name);
        let res;
        if let TokenType::Constant(val) = &cur_tok.token_type {
            res = *val;
        } else {
            self.got_unexpected_token(expected_tok_name, cur_tok);
        }

        self.pos += 1;
        return res;
    }

    fn parse_var(&mut self) -> CIdentifier {
        let expected_tok_name = "identifier";
        let cur_tok = self.get_token(expected_tok_name);
        let res;
        if let TokenType::Identifier(id) = &cur_tok.token_type {
            res = CIdentifier(id.clone());
        } else {
            self.got_unexpected_token(expected_tok_name, cur_tok)
        }

        self.pos += 1;
        return res;
    }

    fn got_unexpected_token(&self, expected_tok_name: &str, cur_tok: &Token) -> ! {
        println!("{:?}", cur_tok.position);
        panic!("Expected an {} but got {:?}", expected_tok_name, cur_tok)
    }
}
