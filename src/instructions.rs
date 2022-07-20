use nom::types::CompleteStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Opcode {
    LOAD,
    HLT,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    JMPB,
    EQ,
    JEQ,
    JNEQ,
    ALOC,
    INC,
    DEC,
    IGL
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 =>  Opcode::LOAD,
            1 =>  Opcode::HLT,
            2 =>  Opcode::ADD,
            3 =>  Opcode::SUB,
            4 =>  Opcode::MUL,
            5 =>  Opcode::DIV,
            6 =>  Opcode::JMP,
            7 =>  Opcode::JMPF,
            8 =>  Opcode::JMPB,
            9 =>  Opcode::EQ,
            10 => Opcode::JEQ,
            11 => Opcode::JNEQ,
            12 => Opcode::ALOC,
            13 => Opcode::INC,
            14 => Opcode::DEC,
            _ =>  Opcode::IGL
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode: opcode
        }
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        match v {
            CompleteStr("load") => Opcode::LOAD,
            CompleteStr("hlt")  => Opcode::HLT,
            CompleteStr("add")  => Opcode::ADD,
            CompleteStr("sub")  => Opcode::SUB,
            CompleteStr("mul")  => Opcode::MUL,
            CompleteStr("div")  => Opcode::DIV,
            CompleteStr("jmp")  => Opcode::JMP,
            CompleteStr("jmpf") => Opcode::JMPF,
            CompleteStr("jmpb") => Opcode::JMPB,
            CompleteStr("eq")   => Opcode::EQ,
            CompleteStr("jeq")  => Opcode::JEQ,
            CompleteStr("jneq") => Opcode::JNEQ,
            CompleteStr("aloc")  => Opcode::ALOC,
            CompleteStr("inc")  => Opcode::INC,
            CompleteStr("dec")  => Opcode::DEC,
            CompleteStr("igl")  => Opcode::IGL,
            _ => Opcode::IGL
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT)
    }
}