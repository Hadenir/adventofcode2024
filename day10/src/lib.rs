use std::collections::{HashMap, HashSet};

use map::TopoMap;

mod map;

fn hiking_trails_scores(map: &TopoMap) -> Vec<usize> {
    let trailheads = map.trailheads();

    let mut possible_trails: Vec<_> = trailheads.into_iter().map(|head| (head, head)).collect();
    let mut visited_peaks = HashMap::<(i64, i64), HashSet<(i64, i64)>>::new();

    while let Some((head, (x, y))) = possible_trails.pop() {
        let Some(height_at_pos) = map.get(x, y) else {
            continue;
        };

        if height_at_pos == 9 {
            visited_peaks
                .entry(head)
                .and_modify(|visited| {
                    visited.insert((x, y));
                })
                .or_insert(HashSet::from([(x, y)]));
            continue;
        }

        let possible_steps = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
        for (x, y) in possible_steps {
            if map.get(x, y) == Some(height_at_pos + 1) {
                possible_trails.push((head, (x, y)));
            }
        }
    }

    visited_peaks
        .into_values()
        .map(|visited_peaks| visited_peaks.len())
        .collect()
}

pub fn solve_part_1(input: &str) -> usize {
    let map: TopoMap = input.parse().expect("Failed to parse puzzle input");
    hiking_trails_scores(&map).into_iter().sum()
}


fn hiking_trails_ratings(map: &TopoMap) -> Vec<usize> {
    let trailheads = map.trailheads();

    let mut possible_trails: Vec<_> = trailheads.into_iter().map(|head| (head, head)).collect();
    let mut ratings = HashMap::<(i64, i64), usize>::new();

    while let Some((head, (x, y))) = possible_trails.pop() {
        let Some(height_at_pos) = map.get(x, y) else {
            continue;
        };

        if height_at_pos == 9 {
            ratings
                .entry(head)
                .and_modify(|rating| {
                    *rating += 1;
                })
                .or_insert(1);
            continue;
        }

        let possible_steps = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
        for (x, y) in possible_steps {
            if map.get(x, y) == Some(height_at_pos + 1) {
                possible_trails.push((head, (x, y)));
            }
        }
    }

    ratings
        .into_values()
        .collect()
}

pub fn solve_part_2(input: &str) -> usize {
    let map: TopoMap = input.parse().expect("Failed to parse puzzle input");
    hiking_trails_ratings(&map).into_iter().sum()
}
