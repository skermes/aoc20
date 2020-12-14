use std::num::ParseFloatError;
use prime_tools::is_u64_prime;
use crate::aoc_error::AocError;

pub const NAME: &str = "Shuttle Search";

pub fn part_one(input: &str) -> Result<String, AocError> {
    let (now, shuttles) = input
        .split_once("\n")
        .ok_or_else(|| AocError::Misc("No newline in input".to_string()))?;

    let now: f64 = now.parse()?;

    let shuttles = shuttles
        .split(',')
        .filter(|s| s != &"x")
        .map(|s| s.parse())
        .collect::<Result<Vec<f64>, ParseFloatError>>()?;

    let (wait, first_shuttle) = shuttles
        .iter()
        .map(|s| {
            let arrival = (now / *s).ceil() * *s;
            let wait = arrival - now;
            (wait as usize, s)
        })
        .min_by_key(|(wait, _)| *wait)
        .unwrap();

    Ok((wait * *first_shuttle as usize).to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let shuttles = input
        .lines()
        .nth(1)
        .ok_or_else(|| AocError::Misc("No second line in input".to_string()))?;

    // Because of the error propagation I can't write this as just as
    // map(...).collect(...) so have a loop.
    let mut congruences: Vec<(u64, u64)> = Vec::new();
    for (remainder, s) in shuttles.split(',').enumerate() {
        if s != "x" {
            let divisor = s.parse()?;
            // If we want a shuttle S to depart at time T, where T > S,
            // that's equivalent to S departing at T - S, since it'll come
            // back around. The modulo here generalizes that to all Ts.
            let remainder = (remainder as u64) % divisor;
            // I'll be blunt, I'm not certain why we need this subtraction.
            // Just from reading the problem it seems like we should be tying
            // to solve a system of congruence relations where each shuttle
            // is a divisor and the offsets from the first one where we want
            // them to arrive are the remainders, but just doing that gives
            // us the wrong answer. Instead we want the remainers to be
            // divisor - offset? Like I said, not sure why this is what we
            // want, but it makes the math work out.
            let remainder = (divisor - remainder) % divisor;
            congruences.push((remainder, divisor));
        }
    }

    // The following algorithm is taken from https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Computation
    // and is one of several ways of solving a system of congruence relations.
    // These all rely on the divisors in the congruences all being pairwise
    // coprime. The input happens to always be generated so that the divisors
    // are all prime, which is both a stronger condition and easier to verify.
    for (_, divisor) in &congruences {
        if !is_u64_prime(*divisor) {
            return Err(AocError::Misc("Non-prime divisor".to_string()));
        }
    }

    // This is the sieving method from the wiki page above. As noted there
    // this isn't the fastest, but it is plenty fast for this input, and my
    // eyes are glazing over reading the wikis explanation for the faster
    // number theory one.

    congruences.sort_unstable_by_key(|(_, divisor)| *divisor);
    congruences.reverse();

    let mut answer = congruences[0].0;
    let mut step = 1;
    for i in 0..(congruences.len() - 1) {
        step *= congruences[i].1;
        let (next_remainder, next_divisor) = congruences[i + 1];

        while answer % next_divisor != next_remainder {
            answer += step;
        }
    }

    Ok(answer.to_string())
}