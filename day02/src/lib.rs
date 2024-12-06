mod parse;

use itertools::Itertools;
use parse::parse_input;

fn check_report_safe<'a>(report: impl IntoIterator<Item = &'a u64>) -> bool {
    let report = report.into_iter().collect_vec();
    let is_increasing = report.iter().tuple_windows().all(|(&&a, &&b)| a <= b);
    let is_decreasing = report.iter().tuple_windows().all(|(&&a, &&b)| a >= b);

    let has_correct_differences = report
        .iter()
        .tuple_windows()
        .all(|(&&a, &&b)| (1..=3).contains(&a.abs_diff(b)));

    (is_decreasing || is_increasing) && has_correct_differences
}

pub fn solve_part_1(input: &str) -> usize {
    let reports = parse_input(input);

    reports
        .into_iter()
        .filter(|report| check_report_safe(report))
        .count()
}

fn check_report_safe_dampened(report: &[u64]) -> bool {
    if check_report_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        if check_report_safe(
            report
                .iter()
                .enumerate()
                .filter_map(|(j, r)| (i != j).then_some(r)),
        ) {
            return true;
        }
    }

    false
}

pub fn solve_part_2(input: &str) -> usize {
    let reports = parse_input(input);

    reports
        .into_iter()
        .filter(|report| check_report_safe_dampened(report))
        .count()
}
