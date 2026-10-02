pub mod ast;

use crate::parser::ast::{
    Expression as CExpression, Function as CFunction, Identifier as CIdentifier,
    Program as CProgram, Statement as CStatement, UnaryOperator as CUnaryOperator,
};
use crate::tacky::ast::{
    Function as TFunction, Instruction as TInstruction, Program as TProgram,
    UnaryOperator as TUnaryOperator, Value as TValue,
};

#[derive(Default)]
pub struct TackyTree {
    pub tmp_var: i32,
}

impl TackyTree {
    pub fn ast_c_to_tacky(&mut self, program: CProgram) -> TProgram {
        let CProgram(func) = program;
        let tfunc = self.func_to_tacky(func);

        return TProgram(tfunc);
    }

    fn func_to_tacky(&mut self, func: CFunction) -> TFunction {
        let CFunction(identifier, statement) = func;
        let CIdentifier(name) = identifier;
        let instructions = self.statement_to_tacky(statement);

        return TFunction(name, instructions);
    }

    fn statement_to_tacky(&mut self, statement: CStatement) -> Vec<TInstruction> {
        let mut instructions = Vec::new();

        match statement {
            CStatement::Return(expression) => {
                let value = self.expr_to_tacky(expression, &mut instructions);
                let instruction = TInstruction::Return(value);
                instructions.push(instruction);
            }
        }

        return instructions;
    }

    fn expr_to_tacky(
        &mut self,
        expression: CExpression,
        instructions: &mut Vec<TInstruction>,
    ) -> TValue {
        match expression {
            CExpression::Constant(val) => return TValue::Constant(val),
            CExpression::Unary(op, box_expr) => {
                let val = self.expr_to_tacky(*box_expr, instructions);
                let var_name = format!("tmp.{}", self.tmp_var);
                let var = TValue::Var(var_name);
                let tacky_op = match op {
                    CUnaryOperator::Complement => TUnaryOperator::Complement,
                    CUnaryOperator::Negate => TUnaryOperator::Negate,
                };
                let instruction = TInstruction::Unary(tacky_op, val, var.clone());
                instructions.push(instruction);
                self.tmp_var += 1;
                return var;
            }
            CExpression::Binary(binary_operator, expression, expression1) => todo!(),
        }
    }
}
