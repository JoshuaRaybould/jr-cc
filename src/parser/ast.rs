// AST for C
#[derive(Debug)]
pub struct Program(pub Function);

#[derive(Debug)]
pub struct Function(pub Identifier, pub Vec<BlockItem>);

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub struct Declaration(pub Identifier, pub Option<Expression>);

#[derive(Debug)]
pub enum BlockItem {
    S(Statement),
    D(Declaration),
}

#[derive(Debug)]
pub enum Expression {
    Constant(i32),
    Var(Identifier),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Assignment(Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    Exp(Expression),
    Null,
    //If(Expression, Box<Statement>, Option<Box<Statement>>),
}

#[derive(Debug)]
pub enum UnaryOperator {
    Complement,
    Negate,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    BitAnd,
    BitOr,
    Xor,
    LeftShift,
    RightShift,
    And,
    Or,
    Equal,
    Neq,
    Le,
    Leq,
    Ge,
    Geq,
    Assign,
}
