// Assembly AST
#[derive(Debug)]
pub struct Program(pub Function);

#[derive(Debug)]
pub struct Function(pub Identifier, pub Vec<Instruction>);

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub enum Instruction {
    Mov(Operand, Operand),
    Ret,
}

#[derive(Debug)]
pub enum Operand {
    Imm(i32),
    Register,
}
