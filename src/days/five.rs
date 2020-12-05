use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Binary Boarding";

#[derive(Debug)]
struct BoardingPass {
    // These could be stored as u8 (row is 0..127) and col is (0..7) but when
    // we find the seat id we have to cast them anyway, this just lets us not
    // have to cast.
    row: u16,
    col: u16
}

fn bit_string(s: &str, zero: char, one: char) -> Result<u16, AocError> {
    s.chars()
        .enumerate()
        .map(|(i, c)| (s.len() - 1 - i, c))
        .try_fold(0, |acc, (shift, c)| {
            let bit = match c {
                _ if c == zero => { 0 },
                _ if c == one => { 1 },
                _ => { return Err(AocError::Misc("Bad bit string char".to_string())) }
            };

            Ok(acc + (bit << shift))
        })
}

impl FromStr for BoardingPass {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = bit_string(&s[..7], 'F', 'B')?;
        let col = bit_string(&s[7..10], 'L', 'R')?;

        Ok(BoardingPass { row, col })
    }
}

impl BoardingPass {
    fn seat_id(&self) -> u16 {
        self.row * 8 + self.col
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let max_id = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<BoardingPass>, AocError>>()?
        .iter()
        .map(|pass| pass.seat_id())
        .max()
        .unwrap();

    Ok(max_id.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let mut seat_ids: Vec<u16> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<BoardingPass>, AocError>>()?
        .iter()
        .map(|pass| pass.seat_id())
        .collect();

    seat_ids.sort();

    seat_ids
        .iter()
        .zip(&seat_ids[1..])
        // Iterator borrows, why you gotta do it to me
        .filter(|(prev, seat)| prev != &&(*seat - 1))
        .map(|(_, seat)| (seat - 1).to_string())
        .next()
        .ok_or_else(|| AocError::Misc("No valid seat".to_string()))
}