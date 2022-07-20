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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT)
    }
}