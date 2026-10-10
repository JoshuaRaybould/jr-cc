use std::collections::HashMap;

use crate::assembly::ast::{
    Function as AFunction, Instruction as AInstruction, Operand as AOperand, Program as AProgram,
};

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
                AInstruction::SetCC(_, dst) => self.replace_pseudo_reg(dst),
                AInstruction::Cmp(src1, src2) => {
                    self.replace_pseudo_reg(src1);
                    self.replace_pseudo_reg(src2);
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
