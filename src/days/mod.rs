use crate::AocError;

type Solution = fn(&str) -> Result<String, AocError>;

pub struct Day {
    pub day: String,
    pub name: String,
    pub part_one: Solution,
    pub part_two: Solution
}

pub fn get_day(day: &str) -> Option<Day> {
    match day {
        _ => None
    }
}
