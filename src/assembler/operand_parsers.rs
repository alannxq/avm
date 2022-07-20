use nom::types::CompleteStr;
use nom::digit;

use crate::assembler::register_parsers::register;
use crate::assembler::Token;

named!(pub operand<CompleteStr, Token>,
    alt!(
        integer_operand |
        register
    )
);

named!(pub integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: digit >>
            (
                Token::IntegerOperand{value: reg_num.parse::<i32>().unwrap()}
            )
        )
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = integer_operand(CompleteStr("#42"));
        assert_eq!(result.is_ok(), true);

        let (_, value) = result.unwrap();
        assert_eq!(value, Token::IntegerOperand{value: 42});
    }
}