pub mod ast;

use crate::parser::ast::{
    BinaryOperator as CBinaryOperator, Expression as CExpression, Function as CFunction,
    Identifier as CIdentifier, Program as CProgram, Statement as CStatement,
    UnaryOperator as CUnaryOperator,
};
use crate::tacky::ast::{
    BinaryOperator as TBinaryOperator, Function as TFunction, Instruction as TInstruction,
    Program as TProgram, UnaryOperator as TUnaryOperator, Value as TValue,
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
            CExpression::Unary(u_op, box_expr) => {
                let val = self.expr_to_tacky(*box_expr, instructions);
                let var_name = self.get_tmp_var_str();
                let var = TValue::Var(var_name);
                let tacky_u_op = self.unary_op_to_tacky(u_op);
                let instruction = TInstruction::Unary(tacky_u_op, val, var.clone());
                instructions.push(instruction);
                return var;
            }
            CExpression::Binary(b_op, box_expr1, box_expr2) => {
                let val1 = self.expr_to_tacky(*box_expr1, instructions);
                let val2 = self.expr_to_tacky(*box_expr2, instructions);
                let var_name = self.get_tmp_var_str();
                let var = TValue::Var(var_name);
                let tacky_b_op = self.binary_op_to_tacky(b_op);
                let instruction = TInstruction::Binary(tacky_b_op, val1, val2, var.clone());
                instructions.push(instruction);
                return var;
            }
        }
    }

    fn get_tmp_var_str(&mut self) -> String {
        let var_name = format!("tmp.{}", self.tmp_var);
        self.tmp_var += 1;
        return var_name;
    }

    fn unary_op_to_tacky(&self, unary_op: CUnaryOperator) -> TUnaryOperator {
        match unary_op {
            CUnaryOperator::Complement => TUnaryOperator::Complement,
            CUnaryOperator::Negate => TUnaryOperator::Negate,
        }
    }

    fn binary_op_to_tacky(&self, binary_op: CBinaryOperator) -> TBinaryOperator {
        match binary_op {
            CBinaryOperator::Add => TBinaryOperator::Add,
            CBinaryOperator::Subtract => TBinaryOperator::Subtract,
            CBinaryOperator::Multiply => TBinaryOperator::Multiply,
            CBinaryOperator::Divide => TBinaryOperator::Divide,
            CBinaryOperator::Remainder => TBinaryOperator::Remainder,
        }
    }
}
