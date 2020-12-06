use crate::aoc_error::AocError;

pub const NAME: &str = "Custom Customs";

fn letter2bit(c: &char) -> u32 {
    1 << (*c as u8 - 97)
}

fn answer_bits(s: &str) -> u32 {
    let mut answers = 0u32;
    for c in s.chars() {
        answers |= letter2bit(&c);
    }
    answers
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let answer_count_sum: u32 = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| answer_bits(person))
                .fold(0, |acc, x| acc | x)
                .count_ones()
        })
        .sum();

    Ok(answer_count_sum.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let answer_count_sum: u32 = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| answer_bits(person))
                .fold(0xFFFFFFFF, |acc, x| acc & x)
                .count_ones()
        })
        .sum();

    Ok(answer_count_sum.to_string())
}