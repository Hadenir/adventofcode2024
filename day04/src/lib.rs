mod grid;

use grid::Grid;

pub fn solve_part_1(input: &str) -> usize {
    let grid: Grid = input.parse().expect("Failed to parse puzzle input");

    let xmas = ["XMAS", "SAMX"];

    let horizontal_xmas: usize = grid
        .get_horizontals()
        .into_iter()
        .map(|row| xmas.iter().map(|x| row.matches(x).count()).sum::<usize>())
        .sum();

    let vertical_xmas: usize = grid
        .get_verticals()
        .into_iter()
        .map(|row| xmas.iter().map(|x| row.matches(x).count()).sum::<usize>())
        .sum();

    let diagonal_xmas = grid
        .iter_subgrid(4, 4)
        .flat_map(|grid| grid.get_diagonals())
        .filter(|string| ["XMAS", "SAMX"].contains(&string.as_str()))
        .count();

    horizontal_xmas + vertical_xmas + diagonal_xmas
}

#[allow(unused)]
pub fn solve_part_2(input: &str) -> usize {
    let grid: Grid = input.parse().expect("Failed to parse puzzle input");

    let mas = ["MAS", "SAM"];

    grid.iter_subgrid(3, 3)
        .map(|grid| grid.get_diagonals())
        .filter(|[diag1, diag2]| {
            mas.contains(&diag1.as_str()) && mas.contains(&diag2.as_str())
        })
        .count()
}
