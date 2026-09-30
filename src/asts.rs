// AST for C
pub mod ast_c {
    #[derive(Debug)]
    pub struct Program(pub Function);

    #[derive(Debug)]
    pub struct Function(pub Identifier, pub Statement);

    #[derive(Debug)]
    pub struct Identifier(pub String);

    #[derive(Debug)]
    pub enum Expression {
        Constant(i32),
        Unary(UnaryOperator, Box<Expression>),
    }

    #[derive(Debug)]
    pub enum Statement {
        Return(Expression),
        //If(Expression, Box<Statement>, Option<Box<Statement>>),
    }

    #[derive(Debug)]
    pub enum UnaryOperator {
        Complement,
        Negate,
    }
}

pub mod ast_a {
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
}
