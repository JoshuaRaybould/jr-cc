use crate::asts::ast_a::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct CodeEmitter {
    program: Program,
    fname: String,
}

impl CodeEmitter {
    fn write_to_file(&self, text: String) -> Result<(), Box<dyn Error>> {
        let mut file = File::options()
            .append(true)
            .create(true)
            .open(&self.fname)?;
        writeln!(&mut file, "{text}")?;
        Ok(())
    }

    pub fn emit_code(&self) {
        let Program(func) = &self.program;
        self.emit_function_code(func);

        let final_line = ".section .note.GNU-stack,\"\",@progbits";
        self.write_to_file(final_line.to_string());
    }

    fn emit_function_code(&self, func: &Function) {
        let Function(identifier, instructions) = func;
        let Identifier(func_name) = identifier;
        let assem_func = format!("\t.global {}\n ", func_name);
        self.write_to_file(assem_func);
        self.write_to_file(format!("{}:", func_name));
        for instruction in instructions {
            match instruction {
                Instruction::Mov(src, dst) => self.emit_mov(src, dst),
                Instruction::Ret => self.emit_ret(),
            }
        }
    }

    // Code emission for instructions
    fn emit_mov(&self, src: &Operand, dst: &Operand) {
        let src_str = self.get_operand_str(src);
        let dst_str = self.get_operand_str(dst);
        let instruction = format!("\tmovl {} {}", src_str, dst_str);
        self.write_to_file(instruction);
    }

    fn emit_ret(&self) {
        self.write_to_file("\tret".to_string());
    }

    // Get operand string
    fn get_operand_str(&self, op: &Operand) -> String {
        match op {
            Operand::Imm(addr) => format!("$<{}>", addr.to_string()),
            Operand::Register => "%eax".to_string(),
        }
    }
}
