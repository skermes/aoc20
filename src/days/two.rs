use std::str::FromStr;
use regex::Regex;
use crate::aoc_error::AocError;

pub const NAME: &str = "Password Philosophy";

struct Rule {
    letter: char,
    x: usize,
    y: usize
}

impl FromStr for Rule {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RULE_PATTERN: Regex =
                Regex::new("([0-9]+)-([0-9]+) ([a-z])").unwrap();
        }

        // Using ok_or instead of ok_or_else can be a significant part of the
        // runtime for a super short program like this. With ok_or_else part
        // two runs in about 600us, with ok_or it's about 1ms. Four string
        // allocations in a loop nearly doubles the time.

        let captures = RULE_PATTERN
            .captures(s)
            .ok_or_else(|| AocError::Misc("Bad rule string".to_string()))?;

        let x = captures.get(1)
            .ok_or_else(|| AocError::Misc("Bad rule string".to_string()))?
            .as_str()
            .parse()?;

        let y = captures.get(2)
            .ok_or_else(|| AocError::Misc("Bad rule string".to_string()))?
            .as_str()
            .parse()?;

        let letter = captures.get(3)
            .ok_or_else(|| AocError::Misc("Bad rule string".to_string()))?
            .as_str()
            .chars()
            .next()
            // This unwrap is safe because this capture can only match if
            // there a char there to iterate over.
            .unwrap();

        Ok(Rule { letter, x, y })
    }
}

impl Rule {
    fn validate_sled_rental(&self, password: &str) -> bool {
        let count = password
            .chars()
            .filter(|c| c == &self.letter)
            .count();
        // Sled rental interprets x and y as min and max counts of letter.
        count >= self.x && count <= self.y
    }

    fn validate_toboggan_corporate(&self, password: &str) -> Result<bool, AocError> {
        // Toboggan Corporate interprets x and y as 1-based indices where
        // exactly one must match letter.
        let first_match = password.chars().nth(self.x - 1);
        let second_match = password.chars().nth(self.y - 1);
        match (first_match, second_match) {
            (Some(char1), Some(char2)) =>
                Ok((char1 == self.letter) ^ (char2 == self.letter)),
            (_, _) => Err(AocError::Misc("Not enough chars in password".to_string()))
        }
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let count = input
        .lines()
        .map(|line| match line.split_once(": ") {
            Some((rule, password)) => {
                let rule = rule.parse::<Rule>()?;
                Ok(rule.validate_sled_rental(password))
            },
            None => Err(AocError::Misc("Bad password line".to_string()))
        })
        .collect::<Result<Vec<bool>, AocError>>()?
        .iter()
        .filter(|valid| **valid)
        .count();

    Ok(count.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let count = input
        .lines()
        .map(|line| match line.split_once(": ") {
            Some((rule, password)) => {
                let rule = rule.parse::<Rule>()?;
                Ok(rule.validate_toboggan_corporate(password)?)
            },
            None => Err(AocError::Misc("Bad password line".to_string()))
        })
        .collect::<Result<Vec<bool>, AocError>>()?
        .iter()
        .filter(|valid| **valid)
        .count();

    Ok(count.to_string())
}