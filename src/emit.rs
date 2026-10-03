use crate::assembly::ast::*;
use std::fs::File;
use std::io::prelude::*;

pub struct CodeEmitter {
    pub program: Program,
    pub fname: String,
}

impl CodeEmitter {
    fn write_to_file(&self, text: String) {
        let mut file = File::options()
            .append(true)
            .create(true)
            .open(&self.fname)
            .expect("Failed to make/open assembly file");
        writeln!(&mut file, "{text}").expect("Failed to write to assembly file");
    }

    pub fn emit_code(&self) {
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.fname)
            .expect("");
        let Program(func) = &self.program;
        self.emit_function_code(func);

        let final_line = ".section .note.GNU-stack,\"\",@progbits";
        self.write_to_file(final_line.to_string());
    }

    fn emit_function_code(&self, func: &Function) {
        let Function(identifier, instructions) = func;
        let Identifier(func_name) = identifier;
        let assem_func = format!("\t.global {}", func_name);
        self.write_to_file(assem_func);
        self.write_to_file(format!("{}:", func_name));
        self.write_to_file("\tpushq %rbp".to_string());
        self.write_to_file("\tmovq %rsp, %rbp".to_string());
        for instruction in instructions {
            match instruction {
                Instruction::Mov(src, dst) => self.emit_mov(src, dst),
                Instruction::Ret => self.emit_ret(),
                Instruction::Unary(unary_operator, operand) => {
                    self.emit_unary(unary_operator, operand);
                }
                Instruction::AllocateStack(offset) => self.emit_aloc(offset),
                Instruction::Binary(b_op, src, dst) => self.emit_binary_op(b_op, src, dst),
                Instruction::Idiv(operand) => {
                    self.write_to_file(format!("\tidivl {}", self.get_operand_str(operand)))
                }
                Instruction::Cdq => self.write_to_file("\tcdq".to_string()),
            }
        }
    }

    // Code emission for instructions
    fn emit_mov(&self, src: &Operand, dst: &Operand) {
        let src_str = self.get_operand_str(src);
        let dst_str = self.get_operand_str(dst);
        let instruction = format!("\tmovl {}, {}", src_str, dst_str);
        self.write_to_file(instruction);
    }

    fn emit_ret(&self) {
        self.write_to_file("\tmovq %rbp, %rsp".to_string());
        self.write_to_file("\tpopq %rbp".to_string());
        self.write_to_file("\tret".to_string());
    }

    // Get operand string
    fn get_operand_str(&self, op: &Operand) -> String {
        match op {
            Operand::Imm(c) => format!("${c}"),
            Operand::Reg(register) => match register {
                Register::AX => "%eax".to_string(),
                Register::R10 => "%r10d".to_string(),
                Register::DX => "%edx".to_string(),
                Register::R11 => "%r11d".to_string(),
                Register::CL => "%cl".to_string(),
                Register::ECX => "%ecx".to_string(),
            },
            Operand::Stack(val) => format!("{val}(%rbp)"),
            Operand::Pseudo(_) => panic!("Psudeoregister made it into the assembly representation"),
        }
    }

    fn emit_unary(&self, unary: &UnaryOperator, operand: &Operand) {
        let operator = match unary {
            UnaryOperator::Negate => "negl".to_string(),
            UnaryOperator::Complement => "notl".to_string(),
        };
        let op = self.get_operand_str(&operand);
        let s = format!("\t{} {}", operator, op);
        self.write_to_file(s);
    }

    fn emit_binary_op(&self, b_op: &BinaryOperator, src: &Operand, dst: &Operand) {
        let operator = match b_op {
            BinaryOperator::Add => "addl".to_string(),
            BinaryOperator::Sub => "subl".to_string(),
            BinaryOperator::Mult => "imull".to_string(),
            BinaryOperator::And => "andl".to_string(),
            BinaryOperator::Xor => "xorl".to_string(),
            BinaryOperator::Or => "orl".to_string(),
            BinaryOperator::Sal => "sall".to_string(),
            BinaryOperator::Sar => "sarl".to_string(),
        };
        let op1 = self.get_operand_str(src);
        let op2 = self.get_operand_str(dst);
        let s = format!("\t{} {}, {}", operator, op1, op2);
        self.write_to_file(s);
    }

    fn emit_aloc(&self, offset: &i32) {
        let s = format!("\tsubq ${}, %rsp", offset);
        self.write_to_file(s);
    }
}
