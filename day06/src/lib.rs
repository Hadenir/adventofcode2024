mod map;

use std::collections::HashSet;

use map::*;

fn get_all_visited_positions(map: &mut Map) -> HashSet<Position> {
    let mut visited_positions = HashSet::new();

    while let Some(guard_position) = map.guard_position() {
        visited_positions.insert(guard_position);
        map.tick()
    }

    visited_positions
}

pub fn solve_part_1(input: &str) -> usize {
    let mut map: Map = input.parse().expect("Failed to parse puzzle input");

    get_all_visited_positions(&mut map).len()
}

fn check_guard_stuck_in_loop(map: &mut Map) -> bool {
    let mut visited_states = HashSet::new();

    while let Some(guard_position) = map.guard_position() {
        let guard_direction = match map.get(guard_position) {
            Some(Tile::Guard(direction)) => direction,
            _ => unreachable!("Tile must be Guard at guard position"),
        };

        let current_state = (guard_position, guard_direction);
        if visited_states.contains(&current_state) {
            return true;
        }

        visited_states.insert(current_state);
        map.tick();
    }

    false
}

pub fn solve_part_2(input: &str) -> usize {
    let map: Map = input.parse().expect("Failed to parse puzzle input");

    let visited_positions_without_obstruction = {
        let mut map = map.clone();
        get_all_visited_positions(&mut map)
    };

    let num_positions = visited_positions_without_obstruction.len();
    let mut result = 0;
    for (i, obstruction_position) in visited_positions_without_obstruction
        .into_iter()
        .enumerate()
    {
        if map.get(obstruction_position).unwrap() != Tile::Empty {
            continue;
        }

        let mut map = map.clone();
        map.set(obstruction_position, Tile::Obstruction);

        if check_guard_stuck_in_loop(&mut map) {
            result += 1;
        }

        if i % 100 == 0 {
            println!("{}% done", i * 100 / num_positions);
        }
    }

    result
}
