use crate::asts::ast_a;
use crate::asts::ast_c;
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

    pub fn parse(&mut self) -> ast_c::Program {
        let func = self.parse_function();
        if self.pos < self.tokens.len() {
            panic!(
                "Not all tokens consumed, currently at {:?}",
                self.tokens.get(self.pos)
            );
        }
        return ast_c::Program(func);
    }

    fn parse_function(&mut self) -> ast_c::Function {
        self.parse_tok(Token::Int);
        let identifier = self.parse_identifier();
        self.parse_tok(Token::OpenParen);
        self.parse_tok(Token::Void); // Likely to need chaging later.
        self.parse_tok(Token::CloseParen);
        self.parse_tok(Token::OpenCurlyBrace);
        let statement = self.parse_statement();
        self.parse_tok(Token::CloseCurlyBrace);
        return ast_c::Function(identifier, statement);
    }

    fn parse_identifier(&mut self) -> ast_c::Identifier {
        let res = match self.tokens.get(self.pos) {
            Some(cur_tok) => {
                if let Token::Identifier(id_name) = cur_tok {
                    ast_c::Identifier(id_name.clone())
                } else {
                    panic!("Expected an identifier, but got {:?}", cur_tok);
                }
            }
            None => panic!("Expected an identifier, but got nothing (parsing ended)"),
        };
        self.pos += 1;
        return res;
    }

    fn parse_statement(&mut self) -> ast_c::Statement {
        self.parse_tok(Token::Return);
        let exp = self.parse_expression();
        self.parse_tok(Token::SemiColon);
        return ast_c::Statement::Return(exp);
    }

    fn parse_expression(&mut self) -> ast_c::Expression {
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
                ast_c::Expression::Unary(
                    ast_c::UnaryOperator::Negate,
                    Box::new(self.parse_expression()),
                )
            }
            Token::Complement => {
                self.parse_tok(Token::Complement);
                ast_c::Expression::Unary(
                    ast_c::UnaryOperator::Complement,
                    Box::new(self.parse_expression()),
                )
            }
            Token::Constant(_) => ast_c::Expression::Constant(self.parse_int()),
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

fn get_indentation(indent: usize) -> String {
    " ".repeat(4).repeat(indent)
}

pub fn pretty_printer(program: ast_c::Program) {
    let indent = 0;
    let ast_c::Program(func) = program;

    println!("Program(");
    pretty_print_function(func, indent + 1);
    println!(")")
}

fn pretty_print_function(func: ast_c::Function, indent: usize) {
    let indentation = get_indentation(indent);
    let ast_c::Function(identifier, statement) = func;

    println!("{}Function(", indentation);
    pretty_print_identifier(identifier, indent + 1);
    pretty_print_statement(statement, indent + 1);
    println!("{})", indentation);
}

fn pretty_print_identifier(identifier: ast_c::Identifier, indent: usize) {
    let indentation = get_indentation(indent);
    let ast_c::Identifier(id_string) = identifier;
    println!("{}name=\"{}\",", indentation, id_string);
}

fn pretty_print_statement(statement: ast_c::Statement, indent: usize) {
    let indentation = get_indentation(indent);

    match statement {
        ast_c::Statement::Return(expression) => {
            println!("{}body=Return(", indentation);
            pretty_print_expression(expression, indent + 1);
            println!("{})", indentation);
        }
    }
}

fn pretty_print_expression(expression: ast_c::Expression, indent: usize) {
    let indentation = get_indentation(indent);
    let expr_str = get_expression_str(expression);

    println!("{}{}", indentation, expr_str);
}

fn get_expression_str(expression: ast_c::Expression) -> String {
    match expression {
        ast_c::Expression::Constant(val) => format!("constant({})", val),
        ast_c::Expression::Unary(unary_operator, expression) => {
            let operator = get_unary_op(unary_operator);
            let expr_str = get_expression_str(*expression);
            format!("{}{}", operator, expr_str)
        }
    }
}

fn get_unary_op(operator: ast_c::UnaryOperator) -> String {
    match operator {
        ast_c::UnaryOperator::Complement => "~".to_string(),
        ast_c::UnaryOperator::Negate => "-".to_string(),
    }
}
/*
pub fn ast_to_ast_a(program: ast_c::Program) -> ast_a::Program {
    let ast_c::Program(func) = program;
    let assembly_func = func_to_ast_a(func);
    return ast_a::Program(assembly_func);
}

fn func_to_ast_a(func: ast_c::Function) -> ast_a::Function {
    let ast_c::Function(identifier, body) = func;
    let ast_c::Identifier(name) = identifier;
    let instructions = statement_to_ast_a(body);
    let identifier_a = ast_a::Identifier(name);
    return ast_a::Function(identifier_a, instructions);
}

fn statement_to_ast_a(body: ast_c::Statement) -> Vec<ast_a::Instruction> {
    let mut instructions = Vec::new();
    match body {
        ast_c::Statement::Return(exp) => match exp {
            ast_c::Expression::Constant(val) => {
                instructions.push(ast_a::Instruction::Mov(
                    ast_a::Operand::Imm(val),
                    ast_a::Operand::Register,
                ));
                instructions.push(ast_a::Instruction::Ret);
            }
        },
    }

    return instructions;
}
    */
