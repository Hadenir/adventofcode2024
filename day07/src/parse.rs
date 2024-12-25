use nom::{
    bytes::complete::tag, character::complete::*, combinator::*, multi::*, sequence::*, Finish,
    IResult,
};

use crate::callibration::Equation;

pub fn parse_input(input: &str) -> Vec<Equation> {
    let (_, equations) = equation_list(input)
        .finish()
        .expect("Failed to parse puzzle input");

    equations
}

fn number(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse)(input)
}

fn operands(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, number)(input)
}

fn equation(input: &str) -> IResult<&str, Equation> {
    map(
        separated_pair(number, tag(": "), operands),
        |(test_value, operands)| Equation {
            test_value,
            operands,
        },
    )(input)
}

fn equation_list(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(line_ending, equation)(input)
}
