use crate::assembly::ast::{
    BinaryOperator as ABinaryOperator, CondCode, Function as AFunction, Identifier as AIdentifier,
    Instruction as AInstruction, Operand as AOperand, Program as AProgram, Register as ARegister,
    UnaryOperator as AUnaryOperator,
};
use crate::tacky::ast::{
    BinaryOperator as TBinaryOperator, Function as TFunction, Instruction as TInstruction,
    Program as TProgram, UnaryOperator as TUnaryOperator, Value as TValue,
};

pub fn fix_instructions(program: &mut AProgram, offset: i32) {
    let AProgram(func) = program;
    fix_func_instructions(func, offset);
}

fn fix_func_instructions(func: &mut AFunction, offset: i32) {
    let AFunction(_, instructions) = func;
    *instructions = fix_each_instruction(instructions, offset);
}

fn fix_each_instruction(instructions: &Vec<AInstruction>, offset: i32) -> Vec<AInstruction> {
    let mut fixed_instrs = Vec::new();
    fixed_instrs.push(AInstruction::AllocateStack(offset)); // First allocate space on the stack!
    for instruction in instructions {
        match instruction {
            AInstruction::Mov(src, dst) => fix_mov_instr(&mut fixed_instrs, instruction, src, dst),
            AInstruction::Binary(b_op, src, dst) => {
                fix_bin_instr(&mut fixed_instrs, instruction, b_op, src, dst);
            }
            AInstruction::Idiv(op) => fix_idiv_instr(&mut fixed_instrs, op),
            _ => fixed_instrs.push(instruction.clone()),
        }
    }
    return fixed_instrs;
}

fn fix_idiv_instr(fixed_instrs: &mut Vec<AInstruction>, op: &AOperand) {
    match op {
        AOperand::Imm(c) => {
            fixed_instrs.push(AInstruction::Mov(
                AOperand::Imm(*c),
                AOperand::Reg(ARegister::R10),
            ));
            fixed_instrs.push(AInstruction::Idiv(AOperand::Reg(ARegister::R10)));
        }
        c => fixed_instrs.push(AInstruction::Idiv(c.clone())),
    }
}

fn fix_mov_instr(
    fixed_instrs: &mut Vec<AInstruction>,
    instruction: &AInstruction,
    src: &AOperand,
    dst: &AOperand,
) {
    match src {
        AOperand::Stack(_) => match dst {
            AOperand::Stack(_) => {
                let instr1 = AInstruction::Mov(src.clone(), AOperand::Reg(ARegister::R10));
                let instr2 = AInstruction::Mov(AOperand::Reg(ARegister::R10), dst.clone());
                fixed_instrs.push(instr1);
                fixed_instrs.push(instr2);
            }
            _ => fixed_instrs.push(instruction.clone()),
        },
        _ => fixed_instrs.push(instruction.clone()),
    }
}

const SIMPLE_FIX_BIN: [ABinaryOperator; 5] = [
    ABinaryOperator::Add,
    ABinaryOperator::Sub,
    ABinaryOperator::And,
    ABinaryOperator::Or,
    ABinaryOperator::Xor,
];

fn fix_bin_instr(
    fixed_instrs: &mut Vec<AInstruction>,
    instruction: &AInstruction,
    b_op: &ABinaryOperator,
    src: &AOperand,
    dst: &AOperand,
) {
    if SIMPLE_FIX_BIN.contains(b_op) {
        fix_simple_binary(fixed_instrs, instruction, b_op, src, dst);
    } else if *b_op == ABinaryOperator::Mult {
        fix_mult(fixed_instrs, instruction, src, dst);
    } else if *b_op == ABinaryOperator::Sal || *b_op == ABinaryOperator::Sar {
        fix_shift(fixed_instrs, instruction, b_op, src, dst);
    }
}

fn fix_simple_binary(
    fixed_instrs: &mut Vec<AInstruction>,
    instruction: &AInstruction,
    b_op: &ABinaryOperator,
    src: &AOperand,
    dst: &AOperand,
) {
    match src {
        AOperand::Stack(_) => match dst {
            AOperand::Stack(_) => {
                let instr1 = AInstruction::Mov(src.clone(), AOperand::Reg(ARegister::R10));
                fixed_instrs.push(instr1);
                if SIMPLE_FIX_BIN.contains(b_op) {
                    let instr2 = AInstruction::Binary(
                        b_op.clone(),
                        AOperand::Reg(ARegister::R10),
                        dst.clone(),
                    );
                    fixed_instrs.push(instr2);
                } else {
                    panic!(
                        "Expected Add, Sub, BitOr, BitAnd, Xor, LeftShift or RightShift but got {:?}",
                        b_op
                    );
                }
            }
            _ => fixed_instrs.push(instruction.clone()),
        },
        _ => fixed_instrs.push(instruction.clone()),
    }
}

fn fix_mult(
    fixed_instrs: &mut Vec<AInstruction>,
    instruction: &AInstruction,
    src: &AOperand,
    dst: &AOperand,
) {
    match dst {
        AOperand::Stack(_) => {
            let instr1 = AInstruction::Mov(dst.clone(), AOperand::Reg(ARegister::R11));
            let instr2 = AInstruction::Binary(
                ABinaryOperator::Mult,
                src.clone(),
                AOperand::Reg(ARegister::R11),
            );
            let instr3 = AInstruction::Mov(AOperand::Reg(ARegister::R11), dst.clone());
            fixed_instrs.push(instr1);
            fixed_instrs.push(instr2);
            fixed_instrs.push(instr3);
        }
        _ => fixed_instrs.push(instruction.clone()),
    }
}

fn fix_shift(
    fixed_instrs: &mut Vec<AInstruction>,
    instruction: &AInstruction,
    b_op: &ABinaryOperator,
    src: &AOperand,
    dst: &AOperand,
) {
    match src {
        AOperand::Stack(_) => {
            let instr1 = AInstruction::Mov(src.clone(), AOperand::Reg(ARegister::ECX));
            fixed_instrs.push(instr1);
            if ABinaryOperator::Sal == *b_op || ABinaryOperator::Sar == *b_op {
                let instr2 =
                    AInstruction::Binary(b_op.clone(), AOperand::Reg(ARegister::CL), dst.clone());
                fixed_instrs.push(instr2);
            } else {
                panic!("Expected LeftShift or RightShift but got {:?}", b_op);
            }
        }
        _ => fixed_instrs.push(instruction.clone()),
    }
}
