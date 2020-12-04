use crate::aoc_error::AocError;

pub const NAME: &str = "Toboggan Trajectory";

struct TreeMap<'a> {
    trees: &'a str,
    height: usize,
    tile_width: usize
}

impl TreeMap<'_> {
    fn new(text: &str) -> TreeMap {
        let height = text.lines().count();
        // Input is known to be rectangular so we can get away with only
        // counting one line.
        let width = text.chars().take_while(|c| c != &'\n').count();

        TreeMap {
            trees: text,
            height,
            tile_width: width
        }
    }

    fn tree_at(&self, row: usize, col: usize) -> bool {
        if row >= self.height {
            // It's convenient for one of the part two seconds to go down
            // past the end of the map.
            return false
        }

        let col = col % self.tile_width;

        // +1 here to account for newline chars
        let i = row * (self.tile_width + 1) + col;
        &self.trees[i..i + 1] == "#"
    }

    fn trees_at_slope(&self, drow: usize, dcol: usize) -> usize {
        (0..self.height)
            .map(|i| (i * drow, i * dcol))
            .filter(|(row, col)| self.tree_at(*row, *col))
            .count()
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let tree_map = TreeMap::new(input);

    Ok(tree_map.trees_at_slope(1, 3).to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let tree_map = TreeMap::new(input);

    let product =
        tree_map.trees_at_slope(1, 1) *
        tree_map.trees_at_slope(1, 3) *
        tree_map.trees_at_slope(1, 5) *
        tree_map.trees_at_slope(1, 7) *
        tree_map.trees_at_slope(2, 1);

    Ok(product.to_string())
}