use std::num::ParseIntError;
use crate::aoc_error::AocError;

pub const NAME: &str = "Rambunctious Recitation";

struct Game {
    turn: u32,
    // We're effectively using this Vec as a hashmap, just without the hashing.
    // A lot of the slots in this vec never get used, but the total memory
    // of the program (precisely measured by eyeballing htop) doesn't seem that
    // much more and it runs ~5 times faster.
    history: Vec<u32>
}

impl Game {
    fn new(max_num: u32) -> Game {
        Game {
            turn: 0,
            history: vec![0; max_num as usize]
        }
    }

    fn speak(&mut self, number: u32) -> u32 {
        self.turn += 1;
        let number = number as usize;

        if self.history[number] > 0 {
            let next = self.turn - self.history[number];
            self.history[number] = self.turn;
            next
        } else {
            self.history[number] = self.turn;
            0
        }
    }

    fn play(&mut self, seed: &[u32], turns: u32) -> u32 {
        let mut next = 0;

        for number in seed {
            next = self.speak(*number);
        }

        while self.turn < (turns - 1) {
            next = self.speak(next);
        }

        next
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let seed = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<u32>, ParseIntError>>()?;

    let mut game = Game::new(2020);
    let last = game.play(&seed, 2020);

    Ok(last.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let seed = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<u32>, ParseIntError>>()?;

    let mut game = Game::new(30000000);
    let last = game.play(&seed, 30000000);

    Ok(last.to_string())
}