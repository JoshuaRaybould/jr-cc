pub mod ast;

use std::collections::HashMap;

use crate::assembly::ast::{
    Function as AFunction, Identifier as AIdentifier, Instruction as AInstruction,
    Operand as AOperand, Program as AProgram, Register as ARegister,
    UnaryOperator as AUnaryOperator,
};
use crate::tacky::ast::{
    Function as TFunction, Instruction as TInstruction, Program as TProgram,
    UnaryOperator as TUnaryOperator, Value as TValue,
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
                let unary_op = match unary_operator {
                    TUnaryOperator::Complement => AUnaryOperator::Complement,
                    TUnaryOperator::Negate => AUnaryOperator::Negate,
                };
                let src_operand = match src {
                    TValue::Constant(c) => AOperand::Imm(*c),
                    TValue::Var(var) => AOperand::Pseudo(var.clone()),
                };
                let dst_operand = match dst {
                    TValue::Constant(c) => AOperand::Imm(*c),
                    TValue::Var(var) => AOperand::Pseudo(var.clone()),
                };
                let cur_instruction = AInstruction::Mov(src_operand, dst_operand.clone());
                a_instructions.push(cur_instruction);
                let cur_instruction = AInstruction::Unary(unary_op, dst_operand);
                a_instructions.push(cur_instruction);
            }
        }
    }

    return a_instructions;
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
            AInstruction::Mov(operand, operand1) => match operand {
                AOperand::Stack(_) => match operand1 {
                    AOperand::Stack(_) => {
                        let instr1 =
                            AInstruction::Mov(operand.clone(), AOperand::Reg(ARegister::R10));
                        let instr2 =
                            AInstruction::Mov(AOperand::Reg(ARegister::R10), operand1.clone());
                        fixed_instrs.push(instr1);
                        fixed_instrs.push(instr2);
                    }
                    _ => fixed_instrs.push(instruction.clone()),
                },
                _ => fixed_instrs.push(instruction.clone()),
            },
            _ => fixed_instrs.push(instruction.clone()),
        }
    }
    return fixed_instrs;
}
