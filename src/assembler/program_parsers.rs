use nom::types::CompleteStr;

use crate::assembler::instruction_parsers::{AssemblerInstruction, instruction_one};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }
}

named!(pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(instruction_one) >>
        (
            Program {
                instructions: instructions
            }
        )
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let result = program(CompleteStr("load $0 #42\n"));
        assert_eq!(result.is_ok(), true);

        let (_, prog) = result.unwrap();

        assert_eq!(prog.instructions.len(), 1);
    }
}