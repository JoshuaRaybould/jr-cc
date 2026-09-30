#[derive(Debug)]
pub struct Program(pub Function);

#[derive(Debug)]
pub struct Function(pub String, pub Vec<Instruction>);

#[derive(Debug)]
pub enum Instruction {
    Return(Value),
    Unary(UnaryOperator, Value, Value),
}

#[derive(Debug)]
pub enum Value {
    Constant(i32),
    Var(String),
}

#[derive(Debug)]
pub enum UnaryOperator {
    Complement,
    Negate,
}
