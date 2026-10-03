pub mod ast;

use std::collections::HashMap;

use crate::assembly::ast::{
    BinaryOperator as ABinaryOperator, Function as AFunction, Identifier as AIdentifier,
    Instruction as AInstruction, Operand as AOperand, Program as AProgram, Register as ARegister,
    UnaryOperator as AUnaryOperator,
};
use crate::tacky::ast::{
    BinaryOperator as TBinaryOperator, Function as TFunction, Instruction as TInstruction,
    Program as TProgram, UnaryOperator as TUnaryOperator, Value as TValue,
};

pub fn full_conversion_t_to_a(program: TProgram) -> AProgram {
    let mut a_program = ast_t_to_ast_a(program);
    // println!("{:?}", a_program);
    let mut register_replacer = PseudoRegReplacer {
        ..Default::default()
    };
    register_replacer.replace_pseudo(&mut a_program);
    let offset = register_replacer.stack_offset;
    // println!("{:?}", a_program);
    fix_instructions(&mut a_program, offset);
    return a_program;
}

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
                let unary_op = unary_op_to_ast_a(unary_operator);
                let src_operand = value_to_ast_a(src);
                let dst_operand = value_to_ast_a(dst);
                let cur_instruction = AInstruction::Mov(src_operand, dst_operand.clone());
                a_instructions.push(cur_instruction);
                let cur_instruction = AInstruction::Unary(unary_op, dst_operand);
                a_instructions.push(cur_instruction);
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
        }
    }

    return a_instructions;
}

fn add_binary_instructions(
    a_instructions: &mut Vec<AInstruction>,
    b_op: &TBinaryOperator,
    src1: AOperand,
    src2: AOperand,
    dst: AOperand,
) {
    let add_sub_mult = [
        TBinaryOperator::Add,
        TBinaryOperator::Subtract,
        TBinaryOperator::Multiply,
    ];
    let div_rem = [TBinaryOperator::Divide, TBinaryOperator::Remainder];
    if add_sub_mult.contains(b_op) {
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
    }
}

fn binary_op_to_ast_a(binary_op: &TBinaryOperator) -> ABinaryOperator {
    match binary_op {
        TBinaryOperator::Add => ABinaryOperator::Add,
        TBinaryOperator::Subtract => ABinaryOperator::Sub,
        TBinaryOperator::Multiply => ABinaryOperator::Mult,
        _ => panic!("Called binary operator conversion on wrong operator"),
    }
}

#[derive(Default)]
pub struct PseudoRegReplacer {
    pub stack_offset: i32,
    pub reg_map: HashMap<String, i32>,
}

impl PseudoRegReplacer {
    pub fn replace_pseudo(&mut self, program: &mut AProgram) {
        let AProgram(func) = program;
        self.replace_func_pseudo(func);
    }

    fn replace_func_pseudo(&mut self, func: &mut AFunction) {
        let AFunction(_, instructions) = func;
        self.replace_instructions_pseudo(instructions);
    }

    fn replace_instructions_pseudo(&mut self, instructions: &mut Vec<AInstruction>) {
        for instruction in instructions {
            match instruction {
                AInstruction::Mov(src, dst) => {
                    self.replace_pseudo_reg(src);
                    self.replace_pseudo_reg(dst);
                }
                AInstruction::Unary(_, operand) => {
                    self.replace_pseudo_reg(operand);
                }
                AInstruction::Binary(_, src, dst) => {
                    self.replace_pseudo_reg(src);
                    self.replace_pseudo_reg(dst);
                }
                AInstruction::Idiv(operand) => {
                    self.replace_pseudo_reg(operand);
                }
                _ => {}
            }
        }
    }

    fn replace_pseudo_reg(&mut self, operand: &mut AOperand) {
        match operand {
            AOperand::Pseudo(pseudo) => {
                let val;
                if self.reg_map.contains_key(pseudo) {
                    val = self.reg_map.get(pseudo).expect("");
                } else {
                    self.stack_offset += 4;
                    val = &self.stack_offset;
                    self.reg_map.insert(pseudo.clone(), *val);
                }
                let replacement: i32 = -val;
                *operand = AOperand::Stack(replacement);
            }
            _ => {}
        }
    }
}

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

fn fix_bin_instr(
    fixed_instrs: &mut Vec<AInstruction>,
    instruction: &AInstruction,
    b_op: &ABinaryOperator,
    src: &AOperand,
    dst: &AOperand,
) {
    match b_op {
        ABinaryOperator::Add => fix_add_sub(fixed_instrs, instruction, b_op, src, dst),
        ABinaryOperator::Sub => fix_add_sub(fixed_instrs, instruction, b_op, src, dst),
        ABinaryOperator::Mult => fix_mult(fixed_instrs, instruction, src, dst),
    }
}

fn fix_add_sub(
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
                if *b_op == ABinaryOperator::Add || *b_op == ABinaryOperator::Sub {
                    let instr2 = AInstruction::Binary(
                        b_op.clone(),
                        AOperand::Reg(ARegister::R10),
                        dst.clone(),
                    );
                    fixed_instrs.push(instr2);
                } else {
                    panic!("Expected Add of Sub but got {:?}", b_op);
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
