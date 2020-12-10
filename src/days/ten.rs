use std::num::ParseIntError;
use crate::aoc_error::AocError;

pub const NAME: &str = "Adapter Array";

pub fn part_one(input: &str) -> Result<String, AocError> {
    let mut nums: Vec<usize> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    nums.sort_unstable();
    nums.insert(0, 0);
    nums.push(nums[nums.len() - 1] + 3);

    let (ones, threes) = nums
        .iter()
        .zip(nums[1..].iter())
        .map(|(adapter, next)| next - adapter)
        .fold((0, 0), |(ones, threes), x| {
            match x {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes)
            }
        });

    Ok((ones * threes).to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let mut nums: Vec<u64> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;

    nums.sort_unstable();
    nums.insert(0, 0);
    nums.push(nums[nums.len() - 1] + 3);

    let diffs: Vec<u64> = nums
        .iter()
        .zip(nums[1..].iter())
        .map(|(x, next)| next - x)
        .collect();

    // Here's how this works. First, notice that if we have two adapters with a
    // gap of three jolts, neither of those adapters can never be removed from
    // the sequence. If we did, we'd create a different of > 3 jolts between the
    // adapters on either side of the hole we make, which is invalid. Thus, we
    // can look for runs of one-difference adapters and consider each of those
    // in isolation. (See comment below for why we're ignoring 2-difference
    // adapters). These runs of consecutive adapters can then be treated in
    // isolation, and the total number of combinations is the product of the
    // combinations we can create frmo each run. Furthermore, each run of
    // consecutive adapters of the same length is identical - four consecutive
    // adapters always add the same number of combinations to our product no
    // matter where in the whole sequence that run appears. Thus, ultimately
    // all we need to do is find the _lengths_ of all the runs of consecutive
    // adapters, map those lengths to the number of combinations they produce,
    // and then multiply those combinations together.

    let runs_of_ones: Vec<u64> = diffs
        .iter()
        .try_fold(vec![0], |mut runs, x| {
            let len = runs.len();
            match x {
                1 => runs[len - 1] += 1,
                3 => runs.push(0),
                // A difference of 2 between adapters isn't invalid according to
                // the problem description, but in looking at the examples and
                // input I discovered that it never appears. The algorithm I'm
                // using is a lot simpler for the case where there are no twos,
                // so that's what I've implemented.
                2 => return Err(AocError::Misc("Found a 2, need a better algorithm".to_string())),
                _ => return Err(AocError::Misc("Found > 3, invalid input".to_string()))
            };
            Ok(runs)
        })?;

    let combinations = runs_of_ones
        .iter()
        .try_fold(1u64, |prod, len| {
            // This is a precomputed table of how many combinations a run of
            // consecutive adapters produces. My input never goes above four
            // in a row, so that's where I stopped here. I've made a couple
            // steps towards working out the general formula but don't have
            // enough time to spend on it.
            let factor = match len {
                0 => 1,
                1 => 1,
                2 => 2,
                3 => 4,
                4 => 7,
                _ => return Err(AocError::Misc("Long run, need a better algorithm".to_string()))
            };

            Ok(prod * factor)
        })?;

    Ok(combinations.to_string())
}