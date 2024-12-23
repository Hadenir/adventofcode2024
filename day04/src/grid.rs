use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
pub struct Grid {
    source: String,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn iter_subgrid(&self, width: usize, height: usize) -> SubgridIter<'_> {
        SubgridIter::new(self, width, height)
    }

    fn get_column(&self, column: usize) -> String {
        assert!(column < self.width);

        self.source
            .chars()
            .skip(column)
            .step_by(self.width)
            .take(self.width)
            .collect()
    }

    fn get_diagonal(&self, diagonal: Diagonal) -> String {
        let step = match diagonal {
            Diagonal::Dexter => self.width + 1,
            Diagonal::Sinister => self.width - 1,
        };

        let skip = match diagonal {
            Diagonal::Dexter => 0,
            Diagonal::Sinister => self.width - 1,
        };

        self.source
            .chars()
            .skip(skip)
            .step_by(step)
            .take(self.height)
            .collect()
    }

    pub fn get_horizontals(&self) -> Vec<String> {
        self.source
            .chars()
            .chunks(self.width)
            .into_iter()
            .map(|chunk| chunk.collect())
            .collect()
    }

    pub fn get_verticals(&self) -> Vec<String> {
        (0..self.width)
            .map(|column| self.get_column(column))
            .collect()
    }

    pub fn get_diagonals(&self) -> [String; 2] {
        [
            self.get_diagonal(Diagonal::Dexter),
            self.get_diagonal(Diagonal::Sinister),
        ]
    }

    fn get_subgrid(&self, x: usize, y: usize, width: usize, height: usize) -> Self {
        let source: String = self
            .source
            .chars()
            .skip(x + y * self.width)
            .chunks(self.width)
            .into_iter()
            .map(|chunk| chunk.take(width))
            .take(height)
            .flatten()
            .collect();

        assert_eq!(source.len(), width * height);

        Self {
            source,
            width,
            height,
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let lines = source.lines().collect_vec();
        let height = lines.len();
        let width = lines[0].len();
        let source: String = lines.into_iter().collect();
        assert_eq!(source.len(), width * height);
        Ok(Self {
            source,
            width,
            height,
        })
    }
}

#[derive(Clone, Copy)]
enum Diagonal {
    Dexter,   // upper-left to lower-right
    Sinister, // upper-right to lower-left
}

pub struct SubgridIter<'a> {
    grid: &'a Grid,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    finished: bool,
}

impl<'a> SubgridIter<'a> {
    fn new(grid: &'a Grid, width: usize, height: usize) -> Self {
        Self {
            grid,
            width,
            height,
            x: 0,
            y: 0,
            finished: false,
        }
    }

    fn move_next(&mut self) {
        self.x += 1;

        if self.x + self.width > self.grid.width {
            self.x = 0;
            self.y += 1;
        }

        if self.y + self.height > self.grid.height {
            self.finished = true;
        }
    }
}

impl<'a> Iterator for SubgridIter<'a> {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let subgrid = self
            .grid
            .get_subgrid(self.x, self.y, self.width, self.height);

        self.move_next();

        Some(subgrid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = "ABC
DEF
GHI";

    #[test]
    fn grid_parse() {
        let grid: Grid = SOURCE.parse().unwrap();

        assert_eq!(grid.source, "ABCDEFGHI");
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
    }

    #[test]
    fn grid_get_column() {
        let grid: Grid = SOURCE.parse().unwrap();

        let column = grid.get_column(1);

        assert_eq!(column, "BEH");
    }

    #[test]
    fn grid_get_diagonals() {
        let grid: Grid = SOURCE.parse().unwrap();

        let dexter = grid.get_diagonal(Diagonal::Dexter);
        let sinister = grid.get_diagonal(Diagonal::Sinister);

        assert_eq!(dexter, "AEI");
        assert_eq!(sinister, "CEG");
    }

    #[test]
    fn grid_get_subgrid() {
        let grid: Grid = SOURCE.parse().unwrap();

        let subgrid = grid.get_subgrid(1, 1, 2, 2);

        assert_eq!(subgrid.source, "EFHI");
        assert_eq!(subgrid.width, 2);
        assert_eq!(subgrid.height, 2);
    }

    #[test]
    fn grid_subgrid_iter() {
        let grid: Grid = SOURCE.parse().unwrap();

        let subrids = grid.iter_subgrid(2, 2).collect_vec();

        assert!(subrids
            .iter()
            .all(|subgrid| subgrid.width == 2 && subgrid.height == 2));
        assert_eq!(subrids.len(), 4);
        assert_eq!(subrids[0].source, "ABDE");
        assert_eq!(subrids[1].source, "BCEF");
        assert_eq!(subrids[2].source, "DEGH");
        assert_eq!(subrids[3].source, "EFHI");
    }
}
