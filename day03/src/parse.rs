use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
    Finish, IResult,
};

use crate::Instruction;

pub fn parse_input(input: &str) -> Vec<Instruction> {
    let (_, instructions) = instructions(input)
        .finish()
        .expect("Failed to parse puzzle input");

    instructions
}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn params(input: &str) -> IResult<&str, (u64, u64)> {
    delimited(
        char('('),
        separated_pair(number, char(','), number),
        char(')'),
    )(input)
}

fn mul_instruction(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("mul"), params), |(a, b)| {
        Instruction::Mul(a, b)
    })(input)
}

fn do_instruction(input: &str) -> IResult<&str, Instruction> {
    map(tag("do()"), |_| Instruction::Do)(input)
}

fn dont_instruction(input: &str) -> IResult<&str, Instruction> {
    map(tag("don't()"), |_| Instruction::Dont)(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(map(
        many_till(
            anychar,
            alt((mul_instruction, do_instruction, dont_instruction)),
        ),
        |(_, i)| i,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_params() {
        let input = "(1,2)";

        let (rem, params) = params(input).unwrap();

        assert_eq!(params, (1, 2));
        assert!(rem.is_empty());
    }

    #[test]
    fn test_parse_mul_instruction() {
        let input = "mul(3,4)";

        let (rem, instruction) = mul_instruction(input).unwrap();

        assert_eq!(instruction, Instruction::Mul(3, 4));
        assert!(rem.is_empty());
    }

    #[test]
    fn test_parse_instructions() {
        let input = "a?dmul(1,3)h27don't()mul(3,7)md.8do()";

        let (rem, instructions) = instructions(input).unwrap();

        assert_eq!(instructions.len(), 4);
        assert_eq!(instructions[0], Instruction::Mul(1, 3));
        assert_eq!(instructions[1], Instruction::Dont);
        assert_eq!(instructions[2], Instruction::Mul(3, 7));
        assert_eq!(instructions[3], Instruction::Do);
        assert!(rem.is_empty());
    }
}
