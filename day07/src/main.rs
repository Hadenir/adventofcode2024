use std::fs;

use day07::*;

fn main() {
    let contents = fs::read_to_string("day07/input.txt")
        .expect("Failed to read puzzle input");

    println!("Part 1: {}", solve_part_1(&contents));

    println!("Part 2: {}", solve_part_2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_1() {
        let sol = solve_part_1(INPUT);

        assert_eq!(sol, 3749);
    }

    #[test]
    fn test_part_2() {
        let sol = solve_part_2(INPUT);

        assert_eq!(sol, 11387);
    }
}
