use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Toboggan Trajectory";

struct TreeMap {
    trees: Vec<Vec<bool>>
}

impl FromStr for TreeMap {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TreeMap {
            trees: s
                .lines()
                .map(|line| line
                    .chars()
                    .map(|c| match c {
                        '#' => Ok(true),
                        '.' => Ok(false),
                        _ => Err(AocError::Misc("Bad tree char".to_string()))
                    })
                    .collect::<Result<Vec<bool>, AocError>>()
                )
                .collect::<Result<Vec<Vec<bool>>, AocError>>()?
        })
    }
}

impl TreeMap {
    fn height(&self) -> usize {
        self.trees.len()
    }

    fn tile_width(&self) -> usize {
        // The input guarantees that the map is rectangular
        self.trees[0].len()
    }

    fn tree_at(&self, row: usize, col: usize) -> bool {
        if row >= self.height() {
            // It's convenient for one of the part two seconds to go down
            // past the end of the map.
            return false
        }

        let col = col % self.tile_width();
        self.trees[row][col]
    }

    fn trees_at_slope(&self, drow: usize, dcol: usize) -> usize {
        (0..self.height())
            .map(|i| (i * drow, i * dcol))
            .filter(|(row, col)| self.tree_at(*row, *col))
            .count()
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let tree_map: TreeMap = input.parse()?;

    Ok(tree_map.trees_at_slope(1, 3).to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let tree_map: TreeMap = input.parse()?;

    let product =
        tree_map.trees_at_slope(1, 1) *
        tree_map.trees_at_slope(1, 3) *
        tree_map.trees_at_slope(1, 5) *
        tree_map.trees_at_slope(1, 7) *
        tree_map.trees_at_slope(2, 1);

    Ok(product.to_string())
}