mod parse;

use parse::parse_input;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

pub fn solve_part_1(input: &str) -> u64 {
    let instructions = parse_input(input);

    instructions
        .into_iter()
        .map(|instruction| match instruction {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

#[allow(unused)]
pub fn solve_part_2(input: &str) -> u64 {
    let instructions = parse_input(input);

    instructions
        .into_iter()
        .fold(
            (0, true),
            |(total, is_enabled), instruction| match instruction {
                Instruction::Do => (total, true),
                Instruction::Dont => (total, false),
                Instruction::Mul(a, b) if is_enabled => (total + a * b, is_enabled),
                _ => (total, is_enabled),
            },
        )
        .0
}
