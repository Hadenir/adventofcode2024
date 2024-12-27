use std::{collections::HashMap, mem};

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(' ')
        .map(|number| number.parse().expect("Input contains only numbers"))
        .collect()
}

fn blink_once(stones: &mut Vec<u64>) {
    let mut i = 0;
    while i < stones.len() {
        let stone = stones[i];

        if stone == 0 {
            stones[i] = 1;
            i += 1;
            continue;
        }

        let num_digits = stone.to_string().len();
        if num_digits % 2 == 0 {
            let half_len: u32 = (num_digits / 2).try_into().unwrap();
            let ten_to_half_len = 10u64.pow(half_len);
            stones[i] = stone / ten_to_half_len;
            stones.insert(i + 1, stone % ten_to_half_len);
            i += 2;
            continue;
        }

        stones[i] *= 2024;
        i += 1;
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let mut stones = parse_input(input);

    for _ in 0..25 {
        blink_once(&mut stones);
    }

    stones.len()
}

fn count_stones_after_blinks(stones: &[u64], num_blinks: usize) -> u128 {
    let mut cache: HashMap<u128, u128> = stones.iter().map(|&stone| (stone.into(), 1)).collect();
    for _ in 0..num_blinks {
        for (stone, num_stones) in mem::take(&mut cache) {
            let mut update_count_in_cache = |stone: u128| {
                cache
                    .entry(stone)
                    .and_modify(|count| {
                        *count += num_stones;
                    })
                    .or_insert(num_stones);
            };

            if stone == 0 {
                update_count_in_cache(1);
                continue;
            }

            let num_digits = stone.to_string().len();
            if num_digits % 2 == 0 {
                let half_len: u32 = (num_digits / 2).try_into().unwrap();
                let ten_to_half_len = 10u128.pow(half_len);
                update_count_in_cache(stone / ten_to_half_len);
                update_count_in_cache(stone % ten_to_half_len);
                continue;
            }

            update_count_in_cache(stone * 2024);
        }
    }

    cache.into_values().sum()
}

pub fn solve_part_2(input: &str) -> u128 {
    let stones = parse_input(input);

    count_stones_after_blinks(&stones, 75)
}
