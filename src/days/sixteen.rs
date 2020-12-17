use std::num::ParseIntError;
use std::str::FromStr;
use std::ops::RangeInclusive;
use std::collections::HashSet;
use crate::aoc_error::AocError;

pub const NAME: &str = "Ticket Translation";

#[derive(Debug, Hash, PartialEq, Eq)]
struct Field {
    name: String,
    rules: Vec<RangeInclusive<usize>>
}

impl FromStr for Field {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((name, rules_str)) = s.split_once(": ") {
            let rules = rules_str
                .split(" or ")
                .map(|s| {
                    if let Some((low, high)) = s.split_once("-") {
                        Ok(low.parse()?..=high.parse()?)
                    } else {
                        Err(AocError::Misc("Invalid range string".to_string()))
                    }
                })
                .collect::<Result<Vec<RangeInclusive<usize>>, AocError>>()?;

            Ok(Field {
                name: name.to_string(),
                rules
            })
        } else {
            Err(AocError::Misc("Invalid field string".to_string()))
        }
    }
}

impl Field {
    fn valid(&self, x: usize) -> bool {
        self.rules.iter().any(|range| range.contains(&x))
    }
}

#[derive(Debug)]
struct Fields(Vec<Field>);

impl FromStr for Fields {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Field>, AocError>>()?;
        Ok(Fields(fields))
    }
}

impl Fields {
    fn valid_any(&self, x: usize) -> bool {
        self.0
            .iter()
            .any(|field| field.valid(x))
    }
}

#[derive(Debug)]
struct Ticket(Vec<usize>);

impl FromStr for Ticket {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<usize>, ParseIntError>>()?;

        Ok(Ticket(nums))
    }
}

impl Ticket {
    fn invalid_sum(&self, fields: &Fields) -> usize {
        self.0
            .iter()
            .filter(|x| !fields.valid_any(**x))
            .sum()
    }

    // One of the invalid numbers is a zero, so just checking invalid_sum == 0
    // won't catch all the invalid fields.
    fn invalid_count(&self, fields: &Fields) -> usize {
        self.0
            .iter()
            .filter(|x| !fields.valid_any(**x))
            .count()
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let parts: Vec<&str> = input.split("\n\n").collect();

    if parts.len() != 3 {
        return Err(AocError::Misc("Wrong number of input parts".to_string()))
    }

    let fields: Fields = parts[0].parse()?;
    let nearby_tickets = parts[2]
        .lines()
        .skip(1)
        .map(|line| line.parse())
        .collect::<Result<Vec<Ticket>, ParseIntError>>()?;

    let answer: usize = nearby_tickets
        .iter()
        .map(|ticket| ticket.invalid_sum(&fields))
        .sum();

    Ok(answer.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let parts: Vec<&str> = input.split("\n\n").collect();

    if parts.len() != 3 {
        return Err(AocError::Misc("Wrong number of input parts".to_string()))
    }

    let fields: Fields = parts[0].parse()?;

    let my_ticket: Ticket = parts[1]
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .parse()?;

    let nearby_tickets = parts[2]
        .lines()
        .skip(1)
        .map(|line| line.parse())
        .collect::<Result<Vec<Ticket>, ParseIntError>>()?;

    let valid_tickets: Vec<&Ticket> = nearby_tickets
        .iter()
        .filter(|ticket| ticket.invalid_count(&fields) == 0)
        .collect();

    let mut candidate_assignments: Vec<(usize, HashSet<&Field>)> = Vec::new();

    for i in 0..fields.0.len() {
        let column_numbers: Vec<usize> = valid_tickets
            .iter()
            .map(|ticket| ticket.0[i])
            .collect();

        let mut candidates = HashSet::new();
        for field in &fields.0 {
            let candidate = column_numbers
                .iter()
                .all(|x| field.valid(*x));

            if candidate {
                candidates.insert(field);
            }
        }

        candidate_assignments.push((i, candidates));
    }

    candidate_assignments.sort_unstable_by_key(|(_, cs)| cs.len());

    let mut assignments: Vec<(usize, &Field)> = Vec::new();
    for (i, candidates) in candidate_assignments.iter_mut() {
        for assignment in &assignments {
            candidates.remove(assignment.1);
        }

        if candidates.len() != 1 {
            println!("too many candidates");
            break;
        }

        let field = candidates.iter().next().unwrap();
        assignments.push((*i, field));
    }

    let product: usize = assignments
        .iter()
        .filter(|(_, field)| field.name.starts_with("departure"))
        .map(|(i, _)| my_ticket.0[*i])
        .product();

    Ok(product.to_string())
}