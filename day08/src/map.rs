use std::{
    collections::HashMap,
    convert::Infallible,
    ops::{Add, Neg, Sub},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: i64,
    y: i64,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for Position {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Antenna(char);

impl From<char> for Antenna {
    fn from(char: char) -> Self {
        Self(char)
    }
}

#[derive(Debug)]
pub struct Map {
    pub width: u64,
    pub height: u64,
    pub antennae: HashMap<Position, Antenna>,
}

impl Map {
    pub fn bounds_check(&self, position: Position) -> bool {
        position.x >= 0
            && position.x < self.width as i64
            && position.y >= 0
            && position.y < self.height as i64
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let lines = string.lines().collect_vec();
        let height = lines.len() as u64;
        let width = lines[0].len() as u64;

        let antennae = lines
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, char)| *char != '.') // Dot represents empty cell.
                    .map(move |(x, char)| {
                        (
                            Position {
                                x: x as i64,
                                y: y as i64,
                            },
                            Antenna(char),
                        )
                    })
            })
            .collect();

        Ok(Self {
            width,
            height,
            antennae,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_map() {
        let input = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
........A.
..........";

        let map: Map = input.parse().unwrap();

        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert_eq!(map.antennae[&Position { x: 4, y: 3 }], Antenna('a'));
        assert_eq!(map.antennae[&Position { x: 5, y: 5 }], Antenna('a'));
        assert_eq!(map.antennae[&Position { x: 8, y: 8 }], Antenna('A'));
    }
}
