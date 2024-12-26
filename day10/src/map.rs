use std::{convert::Infallible, str::FromStr};

use itertools::Itertools;

pub struct TopoMap {
    pub width: usize,
    pub height: usize,
    grid: Vec<u8>,
}

fn xy_to_index(x: i64, y: i64, width: usize) -> usize {
    x as usize + y as usize * width
}

fn index_to_xy(index: usize, width: usize) -> (i64, i64) {
    ((index % width) as i64, (index / width) as i64)
}

impl TopoMap {
    pub fn get(&self, x: i64, y: i64) -> Option<u8> {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            return None;
        }

        Some(self.grid[xy_to_index(x, y, self.width)])
    }

    pub fn trailheads(&self) -> Vec<(i64, i64)> {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, height)| **height == 0)
            .map(|(idx, _)| index_to_xy(idx, self.width))
            .collect()
    }
}

impl FromStr for TopoMap {
    type Err = Infallible;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let lines = string.lines().collect_vec();
        let height = lines.len();
        let width = lines[0].len();

        let grid: Vec<u8> = lines
            .into_iter()
            .flat_map(str::chars)
            .map(|char| char.to_digit(10).unwrap().try_into().unwrap())
            .collect();

        assert_eq!(grid.len(), width * height);

        Ok(Self {
            width,
            height,
            grid,
        })
    }
}
