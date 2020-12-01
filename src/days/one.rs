use std::collections::HashSet;
use std::num::ParseIntError;
use crate::aoc_error::AocError;

pub const NAME: &'static str = "Report Repair";

pub fn part_one(input: &str) -> Result<String, AocError> {
    let nums: HashSet<usize> = input
        .split('\n')
        .map(|line| line.parse())
        .collect::<Result<HashSet<usize>, ParseIntError>>()?;

    let diffs: HashSet<usize> = nums
        .iter()
        .map(|n| 2020 - n)
        .collect();

    let answer: usize = nums
        .intersection(&diffs)
        .product();

    Ok(answer.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let nums: HashSet<usize> = input
        .split('\n')
        .map(|line| line.parse())
        .collect::<Result<HashSet<usize>, ParseIntError>>()?;

    for num in &nums {
        let target = 2020 - num;
        let diffs: HashSet<usize> = nums
            .iter()
            .map(|n| if target > *n { target - n } else { 0 })
            .collect();

        let candidates: Vec<&usize> = nums
            .intersection(&diffs)
            .collect();

        if candidates.len() == 2 {
            let answer: usize = candidates
                .iter()
                // For reasons I don't fully understand, `product` is choking
                // on the type of iter here being &&usize, so I'll just do my
                // own fold.
                .fold(*num, |acc, x| acc * *x);

            return Ok(answer.to_string());
        }
    }

    Err(AocError::Misc("Didn't find a solution".to_string()))
}