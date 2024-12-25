use callibration::{Equation, Operator};
use parse::parse_input;

mod callibration;
mod parse;

fn try_solve_equation(equation: Equation, operations: &[Operator]) -> Option<i64> {
    fn bruteforce(
        operations: &[Operator],
        accumulator: i64,
        operands: &[i64],
        test_value: i64,
    ) -> bool {
        if operands.is_empty() {
            return accumulator == test_value;
        }

        operations.iter().any(|operator| {
            let accumulator = operator.evaluate(accumulator, operands[0]);
            bruteforce(operations, accumulator, &operands[1..], test_value)
        })
    }

    bruteforce(
        operations,
        equation.operands[0],
        &equation.operands[1..],
        equation.test_value,
    )
    .then_some(equation.test_value)
}

pub fn solve_part_1(input: &str) -> i64 {
    let equations = parse_input(input);

    equations
        .into_iter()
        .filter_map(|eq| try_solve_equation(eq, &[Operator::Add, Operator::Multiply]))
        .sum()
}

pub fn solve_part_2(input: &str) -> i64 {
    let equations = parse_input(input);

    equations
        .into_iter()
        .filter_map(|eq| {
            try_solve_equation(
                eq,
                &[Operator::Add, Operator::Multiply, Operator::Concatenate],
            )
        })
        .sum()
}
