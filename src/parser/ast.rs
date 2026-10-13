// AST for C
#[derive(Debug)]
pub struct Program(pub Function);

#[derive(Debug)]
pub struct Function(pub Identifier, pub Vec<BlockItem>);

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub struct Declaration(pub Identifier, pub Option<Expression>);

#[derive(Debug)]
pub enum BlockItem {
    S(Statement),
    D(Declaration),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Constant(i32),
    Var(Identifier),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Assignment(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Return(Expression),
    Exp(Expression),
    Null,
    //If(Expression, Box<Statement>, Option<Box<Statement>>),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Complement,
    Negate,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
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
