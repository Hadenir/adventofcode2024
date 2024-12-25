use std::fs;

use day06::*;

fn main() {
    let contents = fs::read_to_string("day06/input.txt")
        .expect("Failed to read puzzle input");

    println!("Part 1: {}", solve_part_1(&contents));

    println!("Part 2: {}", solve_part_2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_1() {
        let sol = solve_part_1(INPUT);

        assert_eq!(sol, 41);
    }

    #[test]
    fn test_part_2() {
        let sol = solve_part_2(INPUT);

        assert_eq!(sol, 6);
    }
}
