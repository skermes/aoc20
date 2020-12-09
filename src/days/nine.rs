use std::cmp::Ordering::*;
use std::num::ParseIntError;
use crate::aoc_error::AocError;

pub const NAME: &str = "Encoding Error";

// This is the dumb way to solve it, but on my machine and input its time is
// indistinguishable from what should be the faster way using a hashset.
// This code is easier to follow though, so I'm keeping it.
fn first_invalid(input: &[usize], window: usize) -> Result<usize, AocError> {
    for i in window..input.len() {
        let candidate = &input[i];

        let mut is_valid = false;
        for x in &input[(i - window)..i] {
            for y in &input[(i - window)..i] {
                if x != y && x + y == *candidate {
                    is_valid = true;
                    break;
                }
            }
        }

        if !is_valid {
            return Ok(*candidate);
        }
    }

    Err(AocError::Misc("No invalid number in input".to_string()))
}

fn subsequence_sum(input: &[usize], target: usize) -> Result<(usize, usize), AocError> {
    let mut start = 0;
    let mut end = 0;
    let mut running_sum = input[start];

    while end < input.len() {
        match running_sum.cmp(&target) {
            Equal => return Ok((start, end)),
            Less => {
                end += 1;
                running_sum += input[end];
            },
            Greater => {
                running_sum -= input[start];
                start += 1;
            }
        };
    }

    Err(AocError::Misc("No subsequence adds to target".to_string()))
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let nums: Vec<usize> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    let invalid = first_invalid(&nums, 25)?;

    Ok(invalid.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let nums: Vec<usize> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    let invalid = first_invalid(&nums, 25)?;
    let (start, end) = subsequence_sum(&nums, invalid)?;

    let min = nums[start..=end].iter().min().unwrap();
    let max = nums[start..=end].iter().max().unwrap();
    let key = min + max;

    Ok(key.to_string())
}