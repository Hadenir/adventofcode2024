use std::{convert::Infallible, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn step(self, direction: Direction) -> Self {
        use Direction::*;
        let (x, y) = match direction {
            Up => (self.x, self.y - 1),
            Down => (self.x, self.y + 1),
            Left => (self.x - 1, self.y),
            Right => (self.x + 1, self.y),
        };
        Position { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_clockwise(self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Obstruction,
    Guard(Direction),
}

impl From<char> for Tile {
    fn from(char: char) -> Self {
        use Tile::*;

        match char {
            '.' => Empty,
            '#' => Obstruction,
            '^' => Guard(Direction::Up),
            _ => panic!("Encountered invalid tile"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    fn check_bounds(&self, position: Position) -> bool {
        (0..self.width as i64).contains(&position.x)
            && (0..self.height as i64).contains(&position.y)
    }

    pub fn get(&self, position: Position) -> Option<Tile> {
        if !self.check_bounds(position) {
            return None;
        }

        let idx = position.x + position.y * self.width as i64;
        self.tiles.get(idx as usize).copied()
    }

    pub fn set(&mut self, position: Position, tile: Tile) {
        if !self.check_bounds(position) {
            return;
        }

        let idx = position.x + position.y * self.width as i64;
        self.tiles[idx as usize] = tile;
    }

    pub fn guard_position(&self) -> Option<Position> {
        let idx = self
            .tiles
            .iter()
            .position(|&tile| matches!(tile, Tile::Guard(_)))?;

        let x = (idx % self.width) as i64;
        let y = (idx / self.width) as i64;

        Some(Position { x, y })
    }

    pub fn tick(&mut self) {
        let Some(guard_position) = self.guard_position() else {
            return;
        };

        let guard_tile = self.get(guard_position).unwrap();
        let guard_direction = match guard_tile {
            Tile::Guard(direction) => direction,
            _ => unreachable!("Tile must be Guard at guard position"),
        };

        let next_guard_position = guard_position.step(guard_direction);
        let obstruction_ahead = self.get(next_guard_position) == Some(Tile::Obstruction);
        if obstruction_ahead {
            let new_direction = guard_direction.rotate_clockwise();
            let new_tile = Tile::Guard(new_direction);
            self.set(guard_position, new_tile);
        } else {
            self.set(next_guard_position, guard_tile);
            self.set(guard_position, Tile::Empty);
        }
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let lines = string.lines().collect_vec();
        let height = lines.len();
        let width = lines[0].len();

        let tiles = lines
            .into_iter()
            .flat_map(str::chars)
            .map(|char| char.into())
            .collect_vec();

        assert_eq!(tiles.len(), width * height);

        Ok(Self {
            tiles,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string_map() {
        let input = ".#.
#^.
..#";

        let map: Map = input.parse().unwrap();

        assert_eq!(map.width, 3);
        assert_eq!(map.height, 3);
        assert_eq!(map.tiles[0], Tile::Empty);
        assert_eq!(map.tiles[3], Tile::Obstruction);
        assert_eq!(map.tiles[4], Tile::Guard(Direction::Up));
    }
}
