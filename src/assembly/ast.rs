// Assembly AST
#[derive(Debug)]
pub struct Program(pub Function);

#[derive(Debug)]
pub struct Function(pub Identifier, pub Vec<Instruction>);

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub enum Instruction {
    Mov(Operand, Operand),
    Unary(UnaryOperator, Operand),
    Binary(BinaryOperator, Operand, Operand),
    Idiv(Operand),
    Cdq,
    AllocateStack(i32),
    Ret,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Complement,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Imm(i32),
    Reg(Register),
    Pseudo(String),
    Stack(i32),
}

#[derive(Debug, Clone)]
pub enum Register {
    AX,
    DX,
    R10,
    R11,
    CL,  // 8 bit counter register
    ECX, // 32 bit counter register (same as above)
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mult,
    And,
    Xor,
    Or,
    Sal,
    Sar,
}
