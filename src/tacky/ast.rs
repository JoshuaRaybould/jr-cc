#[derive(Debug)]
pub struct Program(pub Function);

#[derive(Debug)]
pub struct Function(pub String, pub Vec<Instruction>);

#[derive(Debug)]
pub enum Instruction {
    Return(Value),
    Unary(UnaryOperator, Value, Value),
    Binary(BinaryOperator, Value, Value, Value),
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
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}
