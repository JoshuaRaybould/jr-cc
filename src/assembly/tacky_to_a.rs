use crate::assembly::ast::{
    BinaryOperator as ABinaryOperator, CondCode, Function as AFunction, Identifier as AIdentifier,
    Instruction as AInstruction, Operand as AOperand, Program as AProgram, Register as ARegister,
    UnaryOperator as AUnaryOperator,
};
use crate::tacky::ast::{
    BinaryOperator as TBinaryOperator, Function as TFunction, Instruction as TInstruction,
    Program as TProgram, UnaryOperator as TUnaryOperator, Value as TValue,
};

pub fn ast_t_to_ast_a(program: TProgram) -> AProgram {
    let TProgram(func) = program;
    let assembly_func = func_to_ast_a(func);
    return AProgram(assembly_func);
}

fn func_to_ast_a(func: TFunction) -> AFunction {
    let TFunction(identifier, instructions) = func;
    let instructions = statement_to_ast_a(instructions);
    let identifier_a = AIdentifier(identifier);
    return AFunction(identifier_a, instructions);
}

fn statement_to_ast_a(instructions: Vec<TInstruction>) -> Vec<AInstruction> {
    let mut a_instructions = Vec::new();

    for instruction in &instructions {
        match instruction {
            TInstruction::Return(val) => {
                let operand = match val {
                    TValue::Constant(c) => AOperand::Imm(*c),
                    TValue::Var(var) => AOperand::Pseudo(var.clone()),
                };
                let cur_instruction = AInstruction::Mov(operand, AOperand::Reg(ARegister::AX));
                a_instructions.push(cur_instruction);
                a_instructions.push(AInstruction::Ret);
            }
            TInstruction::Unary(unary_operator, src, dst) => {
                full_unary_to_ast_a(&mut a_instructions, unary_operator, src, dst);
            }
            TInstruction::Binary(b_operator, src1, src2, dst) => {
                let src1_operand = value_to_ast_a(src1);
                let src2_operand = value_to_ast_a(src2);
                let dst_operand = value_to_ast_a(dst);
                add_binary_instructions(
                    &mut a_instructions,
                    b_operator,
                    src1_operand,
                    src2_operand,
                    dst_operand,
                );
            }
            TInstruction::Copy(src, dst) => {
                let src_operand = value_to_ast_a(src);
                let dst_operand = value_to_ast_a(dst);
                let cur_instruction = AInstruction::Mov(src_operand, dst_operand);
                a_instructions.push(cur_instruction);
            }
            TInstruction::Jump(target) => {
                let cur_instruction = AInstruction::Jmp(AIdentifier(target.clone()));
                a_instructions.push(cur_instruction);
            }
            TInstruction::JumpIfZero(cond, target) => {
                jump_if_zero(true, &mut a_instructions, cond, target)
            }
            TInstruction::JumpIfNotZero(cond, target) => {
                jump_if_zero(false, &mut a_instructions, cond, target)
            }
            TInstruction::Label(id) => {
                let cur_instruction = AInstruction::Label(AIdentifier(id.clone()));
                a_instructions.push(cur_instruction);
            }
        }
    }

    return a_instructions;
}

const SIMPLE_BIN: [TBinaryOperator; 8] = [
    // These get turned into instructions with a mov then the binary operation.
    TBinaryOperator::Add,
    TBinaryOperator::Subtract,
    TBinaryOperator::Multiply,
    TBinaryOperator::Xor,
    TBinaryOperator::BitOr,
    TBinaryOperator::BitAnd,
    TBinaryOperator::LeftShift,
    TBinaryOperator::RightShift,
];

fn add_binary_instructions(
    a_instructions: &mut Vec<AInstruction>,
    b_op: &TBinaryOperator,
    src1: AOperand,
    src2: AOperand,
    dst: AOperand,
) {
    let div_rem = [TBinaryOperator::Divide, TBinaryOperator::Remainder];
    if SIMPLE_BIN.contains(b_op) {
        let op = binary_op_to_ast_a(b_op);
        let instr1 = AInstruction::Mov(src1, dst.clone());
        let instr2 = AInstruction::Binary(op, src2, dst);
        a_instructions.push(instr1);
        a_instructions.push(instr2);
    } else if div_rem.contains(b_op) {
        let instr1 = AInstruction::Mov(src1, AOperand::Reg(ARegister::AX));
        let instr2 = AInstruction::Cdq;
        let instr3 = AInstruction::Idiv(src2);
        let instr4 = match b_op {
            TBinaryOperator::Divide => AInstruction::Mov(AOperand::Reg(ARegister::AX), dst),
            TBinaryOperator::Remainder => AInstruction::Mov(AOperand::Reg(ARegister::DX), dst),
            _ => panic!("Not possible!"),
        };
        a_instructions.push(instr1);
        a_instructions.push(instr2);
        a_instructions.push(instr3);
        a_instructions.push(instr4);
    }
}

fn value_to_ast_a(val: &TValue) -> AOperand {
    match val {
        TValue::Constant(c) => AOperand::Imm(*c),
        TValue::Var(var) => AOperand::Pseudo(var.clone()),
    }
}

fn unary_op_to_ast_a(unary_op: &TUnaryOperator) -> AUnaryOperator {
    match unary_op {
        TUnaryOperator::Complement => AUnaryOperator::Complement,
        TUnaryOperator::Negate => AUnaryOperator::Negate,
        TUnaryOperator::Not => todo!(),
    }
}

fn jump_if_zero(
    equality: bool,
    a_instructions: &mut Vec<AInstruction>,
    cond: &TValue,
    target: &String,
) {
    let condition = value_to_ast_a(cond);
    let instr1 = AInstruction::Cmp(AOperand::Imm(0), condition);
    a_instructions.push(instr1);

    let condition_code = match equality {
        true => CondCode::E,
        false => CondCode::NE,
    };

    let target_id = AIdentifier(target.clone());
    let instr2 = AInstruction::JmpCC(condition_code, target_id);

    a_instructions.push(instr2);
}

fn full_unary_to_ast_a(
    a_instructions: &mut Vec<AInstruction>,
    unary_op: &TUnaryOperator,
    src: &TValue,
    dst: &TValue,
) {
    let src_operand = value_to_ast_a(src);
    let dst_operand = value_to_ast_a(dst);
    match unary_op {
        TUnaryOperator::Not => {
            let instr1 = AInstruction::Cmp(AOperand::Imm(0), src_operand);
            let instr2 = AInstruction::Mov(AOperand::Imm(0), dst_operand.clone());
            let instr3 = AInstruction::SetCC(CondCode::E, dst_operand);
            a_instructions.push(instr1);
            a_instructions.push(instr2);
            a_instructions.push(instr3);
        }
        unary_operator => {
            let unary_op = unary_op_to_ast_a(unary_operator);

            let cur_instruction = AInstruction::Mov(src_operand, dst_operand.clone());
            a_instructions.push(cur_instruction);
            let cur_instruction = AInstruction::Unary(unary_op, dst_operand);
            a_instructions.push(cur_instruction);
        }
    }
}

fn binary_op_to_ast_a(binary_op: &TBinaryOperator) -> ABinaryOperator {
    match binary_op {
        TBinaryOperator::Add => ABinaryOperator::Add,
        TBinaryOperator::Subtract => ABinaryOperator::Sub,
        TBinaryOperator::Multiply => ABinaryOperator::Mult,
        TBinaryOperator::BitAnd => ABinaryOperator::And,
        TBinaryOperator::BitOr => ABinaryOperator::Or,
        TBinaryOperator::Xor => ABinaryOperator::Xor,
        TBinaryOperator::LeftShift => ABinaryOperator::Sal,
        TBinaryOperator::RightShift => ABinaryOperator::Sar,
        _ => panic!(""),
    }
}

fn full_binary_to_ast_a() {}
