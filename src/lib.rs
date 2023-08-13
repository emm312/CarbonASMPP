use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

pub mod lowerer;
pub mod ast;

#[macro_export]
macro_rules! gen_instruction {
    ($type:expr; $($operand:expr)*) => {
        Instruction {
            opcode: $type,
            operands: vec![$($operand,)*]
        }
    }
}