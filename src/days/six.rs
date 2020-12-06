use std::collections::HashSet;
use crate::aoc_error::AocError;

pub const NAME: &str = "Custom Customs";

pub fn part_one(input: &str) -> Result<String, AocError> {
    let answer_count_sum: usize = input
        .split("\n\n")
        .map(|s| {
            let mut answers = HashSet::new();
            for c in s.chars() {
                answers.insert(c);
            }
            // Some groups will have multiple lines, which will make us insert
            // a newline in the loop above. Some groups won't. Always adding a
            // newline lets us skip an if-clause.
            answers.insert('\n');
            answers.len() - 1
        })
        .sum();

    Ok(answer_count_sum.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let answer_count_sum: usize = input
        .split("\n\n")
        .map(|group| {
            group.lines()
                .map(|person| person.chars().collect::<HashSet<char>>())
                .fold_first(|acc, x| {
                    acc
                        .intersection(&x)
                        .map(|&c| c)
                        .collect()
                })
                .unwrap()
                .len()
        })
        .sum();

    Ok(answer_count_sum.to_string())
}