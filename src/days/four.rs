use std::str::FromStr;
use std::collections::HashMap;
use crate::aoc_error::AocError;

pub const NAME: &str = "Passport Processing";

struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>
}

impl<'a> Passport<'a> {
    fn from_str(s: &'a str) -> Result<Passport<'a>, AocError> {
        let fields = s.split_whitespace()
            .map(|field| {
                field
                    .split_once(":")
                    .ok_or_else(|| AocError::Misc("Bad record".to_string()))
            })
            .collect::<Result<HashMap<&str, &str>, AocError>>()?;

        Ok(Passport { fields })
    }
}

impl Passport<'_> {
    // Part one only cares that the keys are there. There are no records with
    // invalid keys, so we can jut check the size of the map.
    fn part_one_valid(&self) -> bool {
        self.fields.len() == 8 ||
        (self.fields.len() == 7 && !self.fields.contains_key("cid"))
    }

    // TODO: If I make fields a more structured type I can make this validation
    // a lot cleaner.
    fn part_two_valid(&self) -> bool {
        if self.fields.len() < 7 {
            return false;
        }

        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if let Some(value) = self.fields.get("byr") {
            match value.parse::<u16>() {
                Err(_) => { return false; },
                Ok(year) => {
                    if !(1920..=2002).contains(&year) {
                        return false;
                    }
                }
            };
        } else {
            return false;
        }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if let Some(value) = self.fields.get("iyr") {
            match value.parse::<u16>() {
                Err(_) => { return false; },
                Ok(year) => {
                    if !(2010..=2020).contains(&year) {
                        return false;
                    }
                }
            };
        } else {
            return false;
        }

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if let Some(value) = self.fields.get("eyr") {
            match value.parse::<u16>() {
                Err(_) => { return false; },
                Ok(year) => {
                    if !(2020..=2030).contains(&year) {
                        return false;
                    }
                }
            };
        } else {
            return false;
        }

        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        if let Some(value) = self.fields.get("hgt") {
            let unit = &value[value.len() - 2..];
            let height: u16 = match &value[..value.len() - 2].parse() {
                Err(_) => { return false; },
                Ok(height) => *height
            };

            if unit == "cm" && !(150..=193).contains(&height) ||
               unit == "in" && !(59..=76).contains(&height) {
                return false;
            }
        } else {
            return false;
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        if let Some(value) = self.fields.get("hcl") {
            if value.len() != 7 {
                return false;
            }

            if &value[..1] != "#" {
                return false;
            }

            if usize::from_str_radix(&value[1..], 16).is_err() {
                return false;
            }
        } else {
            return false;
        }

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        if let Some(value) = self.fields.get("ecl") {
            if !(value == &"amb" || value == &"blu" || value == &"brn" ||
                 value == &"gry" || value == &"grn" || value == &"hzl" ||
                 value == &"oth") {
                return false;
            }
        } else {
            return false;
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        if let Some(value) = self.fields.get("pid") {
            if value.len() != 9 {
                return false;
            }

            if usize::from_str(value).is_err() {
                return false;
            }
        } else {
            return false;
        }

        // cid (Country ID) - ignored, missing or not.

        true
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let valid_count = input
        .split("\n\n")
        .map(|record| Passport::from_str(record))
        .try_fold(0, |acc, passport: Result<Passport, AocError>| {
            match passport {
                Err(err) => Err(err),
                Ok(passport) => if passport.part_one_valid() {
                    Ok(acc + 1)
                } else {
                    Ok(acc)
                }
            }
        })?;

    Ok(valid_count.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let valid_count = input
        .split("\n\n")
        .map(|record| Passport::from_str(record))
        .try_fold(0, |acc, passport: Result<Passport, AocError>| {
            match passport {
                Err(err) => Err(err),
                Ok(passport) => if passport.part_two_valid() {
                    Ok(acc + 1)
                } else {
                    Ok(acc)
                }
            }
        })?;

    Ok(valid_count.to_string())
}