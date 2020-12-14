use crate::aoc_error::AocError;

pub const NAME: &str = "Seating System";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Floor,
    Empty,
    Occupied
}

use State::*;

impl State {
    fn from_char(c: char) -> Result<Self, AocError> {
        match c {
            '.' => Ok(Floor),
            'L' => Ok(Empty),
            '#' => Ok(Occupied),
            _ => Err(AocError::Misc(format!("Invalid state char {}", c)))
        }
    }
}

struct Positions {
    width: usize,
    height: usize,
    row: usize,
    col: usize
}

impl Iterator for Positions {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.height || self.col >= self.width {
            None
        } else {
            let pos = (self.row, self.col);

            self.col += 1;
            if self.col == self.width {
                self.col = 0;
                self.row += 1;
            }

            Some(pos)
        }
    }
}

impl Positions {
    fn new(width: usize, height: usize) -> Positions {
        Positions {
            width,
            height,
            row: 0,
            col: 0
        }
    }
}

const DIRECTIONS: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1),
                                         ( 0, -1),          ( 0, 1),
                                         ( 1, -1), (1,  0), ( 1, 1)];

fn neighbors_p1(row: usize, col: usize, width: usize, height: usize, tiles: &[State]) -> Vec<usize> {
    let mut neighbors = Vec::new();
    for dir in DIRECTIONS.iter() {
        let other_row = row as isize + dir.0;
        let other_col = col as isize + dir.1;

        if other_row >= 0 && other_col >= 0 && other_row < height as isize && other_col < width as isize {
            let other_index = other_row as usize * width + other_col as usize;
            let other = tiles[other_index];

            if other == Empty {
                neighbors.push(other_index);
            }
        }
    }

    neighbors
}

fn neighbors_p2(row: usize, col: usize, width: usize, height: usize, tiles: &[State]) -> Vec<usize> {
    let mut neighbors = Vec::new();

    for direction in DIRECTIONS.iter() {
        let mut other_row = row as isize + direction.0;
        let mut other_col = col as isize + direction.1;
        loop {
            if other_row < 0 || other_col < 0 || other_row >= height as isize || other_col >= width as isize {
                break;
            }

            let other_index = other_row as usize * width + other_col as usize;
            let other = tiles[other_index];
            if other == Empty {
                neighbors.push(other_index);
                break;
            }

            other_row += direction.0;
            other_col += direction.1;
        }
    }

    neighbors
}

struct WaitingArea {
    tiles: Vec<State>,
    neighbors: Vec<Vec<usize>>,
    width: usize,
    height: usize
}

impl WaitingArea {
    fn read<F>(s: &str, neighbor_fn: F) -> Result<Self, AocError>
        where F: Fn(usize, usize, usize, usize, &[State]) -> Vec<usize>
    {
        let mut lines = 0;
        let mut cols = 0;
        let mut tiles = Vec::new();
        for c in s.chars() {
            if lines == 0 && c != '\n' {
                cols += 1;
            }

            if c == '\n' {
                lines += 1;
            } else {
                tiles.push(State::from_char(c)?);
            }
        }
        // Extra +1 for the trimmed final newline.
        lines += 1;

        let mut neighbors = Vec::with_capacity(tiles.len());
        for (row, col) in Positions::new(cols, lines) {
            neighbors.push(neighbor_fn(row, col, cols, lines, &tiles));
        }

        Ok(WaitingArea {
            tiles,
            neighbors,
            width: cols,
            height: lines
        })
    }
}

impl WaitingArea {
    fn positions(&self) -> Positions {
        Positions::new(self.width, self.height)
    }

    fn state_at(&self, row: isize, col: isize) -> State {
        // Everything outside the area bounds can be treated as a empty for
        // the purposes of "how many full seats are adjacent to me".
        if row < 0 || col < 0 || row >= self.height as isize || col >= self.width as isize {
            Empty
        } else {
            let index = row as usize * self.width + col as usize;
            self.tiles[index]
        }
    }

    fn flip(&mut self, row: isize, col: isize) {
        if row < 0 || col < 0 || row >= self.height as isize || col >= self.width as isize {
            println!("Trying to flip outside bounds, probably an error");
        } else {
            let index = row as usize * self.width + col as usize;
            if self.tiles[index] == Occupied {
                self.tiles[index] = Empty;
            } else if self.tiles[index] == Empty {
                self.tiles[index] = Occupied;
            }
        }
    }

    fn tick(&mut self, leave_threshold: usize) {
        let mut to_flip = Vec::new();

        // TODO: Iterators
        for (row, col) in self.positions() {
            let here = self.state_at(row as isize, col as isize);

            if here == Floor {
                continue;
            }

            let mut occupied = 0;
            for other_index in &self.neighbors[row * self.width + col] {
                if self.tiles[*other_index] == Occupied {
                    occupied += 1;
                }
            }

            if here == Empty && occupied == 0 ||
               here == Occupied && occupied >= leave_threshold{
                to_flip.push((row, col));
            }
        }

        for (row, col) in to_flip {
            self.flip(row as isize, col as isize);
        }
    }

    fn total_occupied(&self) -> usize {
        self.tiles
            .iter()
            .filter(|&&state| state == Occupied)
            .count()
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let mut area = WaitingArea::read(input, neighbors_p1)?;

    let mut prev_occupied = area.total_occupied();
    loop {
        area.tick(4);
        let now_occupied = area.total_occupied();
        if now_occupied == prev_occupied {
            break;
        }
        prev_occupied = now_occupied;
    }

    Ok(prev_occupied.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let mut area = WaitingArea::read(input, neighbors_p2)?;

    let mut prev_occupied = area.total_occupied();
    loop {
        area.tick(5);
        let now_occupied = area.total_occupied();
        if now_occupied == prev_occupied {
            break;
        }
        prev_occupied = now_occupied;
    }

    Ok(prev_occupied.to_string())
}