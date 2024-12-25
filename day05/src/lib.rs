pub(crate) mod manual;
mod parse;

use itertools::Itertools;
use manual::*;
use parse::parse_input;

fn check_rule(rule: &OrderingRule, update: &[Page]) -> bool {
    let before_idx = update
        .iter()
        .position(|&page| page == rule.before)
        .unwrap_or(usize::MIN);
    let after_idx = update
        .iter()
        .position(|&page| page == rule.after)
        .unwrap_or(usize::MAX);
    before_idx <= after_idx
}

fn check_rules(rules: &[OrderingRule], update: &[Page]) -> bool {
    rules.iter().all(|rule| check_rule(rule, update))
}

pub fn solve_part_1(input: &str) -> u64 {
    let puzzle_input = parse_input(input);

    puzzle_input
        .updates
        .iter()
        .filter(|update| check_rules(&puzzle_input.ordering_rules, update))
        .map(|update| {
            let middle_idx = update.len() / 2;
            update[middle_idx]
        })
        .sum()
}

fn fix_update(rules: &[OrderingRule], update: &mut [Page]) {
    for rule in rules.iter().cycle() {
        let Some(before_idx) = update.iter().position(|&page| page == rule.before) else {
            continue;
        };
        let Some(after_idx) = update.iter().position(|&page| page == rule.after) else {
            continue;
        };

        if before_idx <= after_idx {
            continue;
        }

        // Move element at `before_idx` before element at `after_index`, to fullfil the rule.
        for i in (after_idx..before_idx).rev() {
            update.swap(i, i + 1);
        }

        // We're done when all rules are fullfilled.
        if check_rules(rules, update) {
            break;
        }
    }
}

pub fn solve_part_2(input: &str) -> u64 {
    let puzzle_input = parse_input(input);

    puzzle_input
        .updates
        .into_iter()
        .filter(|update| !check_rules(&puzzle_input.ordering_rules, update))
        .update(|update| fix_update(&puzzle_input.ordering_rules, update))
        .map(|update| {
            let middle_idx = update.len() / 2;
            update[middle_idx]
        })
        .sum()
}
