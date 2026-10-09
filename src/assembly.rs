pub mod ast;
pub mod fix;
pub mod pseudoreg_replace;
pub mod tacky_to_a;

use crate::assembly::ast::Program as AProgram;
use crate::tacky::ast::Program as TProgram;

use crate::assembly::fix::fix_instructions;
use crate::assembly::pseudoreg_replace::PseudoRegReplacer;

pub fn full_conversion_t_to_a(program: TProgram) -> AProgram {
    let mut a_program = tacky_to_a::ast_t_to_ast_a(program);
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
