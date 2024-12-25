use nom::{
    character::complete::*, combinator::*, multi::*, sequence::*,
    Finish, IResult,
};

use crate::manual::*;

pub struct PuzzleInput {
    pub ordering_rules: Vec<OrderingRule>,
    pub updates: Vec<Vec<Page>>,
}

pub fn parse_input(input: &str) -> PuzzleInput {
    let (_, puzzle_input) = puzzle_input(input)
        .finish()
        .expect("Failed to parse puzzle input");

    puzzle_input
}

fn page(input: &str) -> IResult<&str, Page> {
    map_res(digit1, str::parse)(input)
}

fn ordering_rule(input: &str) -> IResult<&str, OrderingRule> {
    map(separated_pair(page, char('|'), page), |(before, after)| {
        OrderingRule { before, after }
    })(input)
}

fn ordering_rules(input: &str) -> IResult<&str, Vec<OrderingRule>> {
    separated_list1(line_ending, ordering_rule)(input)
}

fn update(input: &str) -> IResult<&str, Vec<Page>> {
    separated_list1(char(','), page)(input)
}

fn update_list(input: &str) -> IResult<&str, Vec<Vec<Page>>> {
    separated_list1(line_ending, update)(input)
}

fn puzzle_input(input: &str) -> IResult<&str, PuzzleInput> {
    map(
        separated_pair(ordering_rules, count(line_ending, 2), update_list),
        |(ordering_rules, updates)| PuzzleInput {
            ordering_rules,
            updates,
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ordering_rule() {
        let input = "12|34";

        let (rem, ordering_rule) = ordering_rule(input).unwrap();

        assert!(rem.is_empty());
        assert_eq!(ordering_rule.before, 12);
        assert_eq!(ordering_rule.after, 34);
    }

    #[test]
    fn parse_update() {
        let input = "11,22,33";

        let (rem, update) = update(input).unwrap();

        assert!(rem.is_empty());
        assert_eq!(update.len(), 3);
        assert_eq!(update[0], 11);
        assert_eq!(update[1], 22);
        assert_eq!(update[2], 33);
    }
    #[test]
    fn parse_puzzle_input() {
        let input = "1|2
2|3

1,2,3";

        let (rem, puzzle_input) = puzzle_input(input).unwrap();

        assert!(rem.is_empty());
        assert_eq!(puzzle_input.ordering_rules.len(), 2);
        assert_eq!(puzzle_input.ordering_rules[0].before, 1);
        assert_eq!(puzzle_input.ordering_rules[0].after, 2);
        assert_eq!(puzzle_input.ordering_rules[1].before, 2);
        assert_eq!(puzzle_input.ordering_rules[1].after, 3);
        assert_eq!(puzzle_input.updates.len(), 1);
        assert_eq!(puzzle_input.updates[0][0], 1);
        assert_eq!(puzzle_input.updates[0][1], 2);
        assert_eq!(puzzle_input.updates[0][2], 3);
    }
}
