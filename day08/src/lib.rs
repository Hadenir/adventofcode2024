use std::collections::HashSet;

use itertools::Itertools;
use map::{Map, Position};

mod map;

fn determine_antinodes(map: &Map) -> HashSet<Position> {
    let grouped_antennae = map
        .antennae
        .iter()
        .map(|(position, antenna)| (antenna, position))
        .into_group_map();

    let mut antinodes = HashSet::new();

    for (_antenna, positions) in grouped_antennae {
        for (&pos1, &pos2) in positions.into_iter().tuple_combinations() {
            let offset = pos2 - pos1;
            for antinode_pos in [pos1 - offset, pos2 + offset] {
                if map.bounds_check(antinode_pos) {
                    antinodes.insert(antinode_pos);
                }
            }
        }
    }

    antinodes
}

pub fn solve_part_1(input: &str) -> usize {
    let map: Map = input.parse().expect("Failed to parse puzzle input");

    determine_antinodes(&map).len()
}

fn determine_t_antinodes(map: &Map) -> HashSet<Position> {
    let grouped_antennae = map
        .antennae
        .iter()
        .map(|(position, antenna)| (antenna, position))
        .into_group_map();

    let mut antinodes = HashSet::new();

    for (_antenna, positions) in grouped_antennae {
        for (&pos1, &pos2) in positions.into_iter().tuple_combinations() {
            let offset = pos2 - pos1;

            let mut antinode = pos1;
            while map.bounds_check(antinode) {
                antinodes.insert(antinode);
                antinode = antinode - offset;
            }

            antinode = pos2;
            while map.bounds_check(antinode) {
                antinodes.insert(antinode);
                antinode = antinode + offset;
            }
        }
    }

    antinodes
}

pub fn solve_part_2(input: &str) -> usize {
    let map: Map = input.parse().expect("Failed to parse puzzle input");

    determine_t_antinodes(&map).len()
}
