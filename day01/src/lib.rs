mod parse;

use parse::parse_input;

pub fn solve_part_1(input: &str) -> u64 {
    let (mut list1, mut list2) = parse_input(input);
    assert_eq!(list1.len(), list2.len());

    list1.sort();
    list2.sort();

    list1
        .into_iter()
        .zip(list2)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

pub fn solve_part_2(input: &str) -> u64 {
    let (list1, list2) = parse_input(input);
    assert_eq!(list1.len(), list2.len());

    list1
        .into_iter()
        .map(|l| {
            let count = list2.iter().filter(|&&r| l == r).count() as u64;
            l * count
        })
        .sum()
}
