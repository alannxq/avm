use crate::assembler::Token;
use crate::assembler::opcode_parsers::*;
use crate::assembler::operand_parsers::{integer_operand, operand};
use crate::assembler::register_parsers::register;
use crate::assembler::directive_parsers::directive;
use crate::assembler::label_parsers::label_declaration;

use crate::instructions::Opcode;

use nom::types::CompleteStr;
use nom::multispace;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode:    Option<Token>,
    pub label:     Option<Token>,
    pub directive: Option<Token>,
    pub operand1:  Option<Token>,
    pub operand2:  Option<Token>,
    pub operand3:  Option<Token>
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Some(Token::Op { code }) => match code {
                _ => {
                    results.push(code as u8)
                }
            },
            _ => {
                println!("Non-opcode found in the opcode field");
                std::process::exit(1);
            }
        };

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(x) => AssemblerInstruction::extract_operand(x, &mut results),
                None => {}
            }
        }

        while results.len() < 4 {
            results.push(0);
        }

        results
    }

    fn extract_operand(token: &Token, results: &mut Vec<u8>) {
        match token {
            Token::Register {reg_num} => {
                results.push(*reg_num)
            },
            Token::IntegerOperand {value} => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            },
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        }
    }
}

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            instruction_combined |
            directive
        ) >>
        (
            ins
        )
    )
);

// named!(instruction_one<CompleteStr, AssemblerInstruction>,
//     do_parse!(
//         o: opcode >>
//         r: register >>
//         i: integer_operand >>
//         (
//             AssemblerInstruction {
//                 opcode: Some(o),
//                 operand1: Some(r),
//                 operand2: Some(i),
//                 operand3: None
//             }
//         )
//     )
// );

// named!(instruction_two<CompleteStr, AssemblerInstruction>,
//     do_parse!(
//         o: opcode >>
//         opt!(multispace) >>
//         (
//             AssemblerInstruction {
//                 opcode: o,
//                 operand1: None,
//                 operand2: None,
//                 operand3: None
//             }
//         )
//     )
// );

named!(instruction_combined<CompleteStr, AssemblerInstruction>,
    do_parse!(
        l: opt!(label_declaration) >>
        o: opcode >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        (
            AssemblerInstruction {
                opcode: Some(o),
                label: l,
                directive: None,
                operand1: o1,
                operand2: o2,
                operand3: o3,
            }
        )
    )
);


mod tests {
    use super::*;

    // #[test]
    // fn test_parse_instruction_form_one() {
    //     let result = instruction_one(CompleteStr("load $0 #100\n"));
    //     assert_eq!(
    //         result,
    //         Ok((
    //             CompleteStr(""),
    //             AssemblerInstruction {
    //                 opcode: Token::Op {code: Opcode::LOAD},
    //                 operand1: Some(Token::Register {reg_num: 0}),
    //                 operand2: Some(Token::IntegerOperand {value: 100}),
    //                 operand3: None
    //             }
    //         ))
    //     )
    // }

    // #[test]
    // fn test_parse_instruction_form_two() {
    //     let result = instruction_two(CompleteStr("hlt\n"));
    //     assert_eq!(
    //         result,
    //         Ok((
    //             CompleteStr(""),
    //             AssemblerInstruction {
    //                 opcode: Token::Op {code: Opcode::HLT},
    //                 operand1: None,
    //                 operand2: None,
    //                 operand3: None
    //             }
    //         ))
    //     )
    // }

    #[test]
    fn test_str_to_opcode() {
        let opcode = Opcode::from(CompleteStr("load"));
        assert_eq!(opcode, Opcode::LOAD);
        let opcode = Opcode::from(CompleteStr("illegal"));
        assert_eq!(opcode, Opcode::IGL);
    }
}