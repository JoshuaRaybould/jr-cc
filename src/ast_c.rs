// AST for C
#[derive(Debug)]
pub struct Program(pub Function);

#[derive(Debug)]
pub struct Function(pub Identifier, pub Statement);

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub enum Expression {
    Constant(i32),
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    //If(Expression, Box<Statement>, Option<Box<Statement>>),
}
