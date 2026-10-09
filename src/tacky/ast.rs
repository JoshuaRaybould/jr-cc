#[derive(Debug)]
pub struct Program(pub Function);

#[derive(Debug)]
pub struct Function(pub String, pub Vec<Instruction>);

#[derive(Debug)]
pub enum Instruction {
    Return(Value),
    Unary(UnaryOperator, Value, Value),
    Binary(BinaryOperator, Value, Value, Value),
    Copy(Value, Value),
    Jump(String),
    JumpIfZero(Value, String),
    JumpIfNotZero(Value, String),
    Label(String),
}

#[derive(Debug, Clone)]
pub enum Value {
    Constant(i32),
    Var(String),
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
