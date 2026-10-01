pub mod ast;

use crate::assembly::ast::{
    Function as AFunction, Identifier as AIdentifier, Instruction as AInstruction,
    Operand as AOperand, Program as AProgram,
};
use crate::parser::ast::{
    Expression as CExpression, Function as CFunction, Identifier as CIdentifier,
    Program as CProgram, Statement as CStatement, UnaryOperator as CUnaryOperator,
};

pub fn ast_c_to_ast_a(program: CProgram) -> AProgram {
    let CProgram(func) = program;
    let assembly_func = func_to_ast_a(func);
    return AProgram(assembly_func);
}

fn func_to_ast_a(func: CFunction) -> AFunction {
    let CFunction(identifier, body) = func;
    let CIdentifier(name) = identifier;
    let instructions = statement_to_ast_a(body);
    let identifier_a = AIdentifier(name);
    return AFunction(identifier_a, instructions);
}

fn statement_to_ast_a(body: CStatement) -> Vec<AInstruction> {
    let mut instructions = Vec::new();
    match body {
        CStatement::Return(exp) => match exp {
            CExpression::Constant(val) => {
                instructions.push(AInstruction::Mov(AOperand::Imm(val), AOperand::Register));
                instructions.push(AInstruction::Ret);
            }
            CExpression::Unary(unary_operator, expression) => {
                todo!("Unary operators not yet supported! my bad ;)")
            }
        },
    }

    return instructions;
}
