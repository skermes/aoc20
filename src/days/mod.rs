mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
mod seven;
mod eight;
mod nine;
mod ten;
mod eleven;
mod twelve;
mod thirteen;
mod fourteen;
mod fifteen;
mod sixteen;
mod seventeen;
mod eighteen;
mod nineteen;
mod twenty;
mod twentyone;
mod twentytwo;
mod twentythree;
mod twentyfour;
mod twentyfive;

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
        "1" => Some(Day {
            day: "1".to_string(),
            name: one::NAME.to_string(),
            part_one: one::part_one,
            part_two: one::part_two
        }),
        "2" => Some(Day {
            day: "2".to_string(),
            name: two::NAME.to_string(),
            part_one: two::part_one,
            part_two: two::part_two
        }),
        "3" => Some(Day {
            day: "3".to_string(),
            name: three::NAME.to_string(),
            part_one: three::part_one,
            part_two: three::part_two
        }),
        "4" => Some(Day {
            day: "4".to_string(),
            name: four::NAME.to_string(),
            part_one: four::part_one,
            part_two: four::part_two
        }),
        "5" => Some(Day {
            day: "5".to_string(),
            name: five::NAME.to_string(),
            part_one: five::part_one,
            part_two: five::part_two
        }),
        "6" => Some(Day {
            day: "6".to_string(),
            name: six::NAME.to_string(),
            part_one: six::part_one,
            part_two: six::part_two
        }),
        "7" => Some(Day {
            day: "7".to_string(),
            name: seven::NAME.to_string(),
            part_one: seven::part_one,
            part_two: seven::part_two
        }),
        "8" => Some(Day {
            day: "8".to_string(),
            name: eight::NAME.to_string(),
            part_one: eight::part_one,
            part_two: eight::part_two
        }),
        "9" => Some(Day {
            day: "9".to_string(),
            name: nine::NAME.to_string(),
            part_one: nine::part_one,
            part_two: nine::part_two
        }),
        "10" => Some(Day {
            day: "10".to_string(),
            name: ten::NAME.to_string(),
            part_one: ten::part_one,
            part_two: ten::part_two
        }),
        "11" => Some(Day {
            day: "11".to_string(),
            name: eleven::NAME.to_string(),
            part_one: eleven::part_one,
            part_two: eleven::part_two
        }),
        "12" => Some(Day {
            day: "12".to_string(),
            name: twelve::NAME.to_string(),
            part_one: twelve::part_one,
            part_two: twelve::part_two
        }),
        "13" => Some(Day {
            day: "13".to_string(),
            name: thirteen::NAME.to_string(),
            part_one: thirteen::part_one,
            part_two: thirteen::part_two
        }),
        "14" => Some(Day {
            day: "14".to_string(),
            name: fourteen::NAME.to_string(),
            part_one: fourteen::part_one,
            part_two: fourteen::part_two
        }),
        "15" => Some(Day {
            day: "15".to_string(),
            name: fifteen::NAME.to_string(),
            part_one: fifteen::part_one,
            part_two: fifteen::part_two
        }),
        "16" => Some(Day {
            day: "16".to_string(),
            name: sixteen::NAME.to_string(),
            part_one: sixteen::part_one,
            part_two: sixteen::part_two
        }),
        "17" => Some(Day {
            day: "17".to_string(),
            name: seventeen::NAME.to_string(),
            part_one: seventeen::part_one,
            part_two: seventeen::part_two
        }),
        "18" => Some(Day {
            day: "18".to_string(),
            name: eighteen::NAME.to_string(),
            part_one: eighteen::part_one,
            part_two: eighteen::part_two
        }),
        "19" => Some(Day {
            day: "19".to_string(),
            name: nineteen::NAME.to_string(),
            part_one: nineteen::part_one,
            part_two: nineteen::part_two
        }),
        "20" => Some(Day {
            day: "20".to_string(),
            name: twenty::NAME.to_string(),
            part_one: twenty::part_one,
            part_two: twenty::part_two
        }),
        "21" => Some(Day {
            day: "21".to_string(),
            name: twentyone::NAME.to_string(),
            part_one: twentyone::part_one,
            part_two: twentyone::part_two
        }),
        "22" => Some(Day {
            day: "22".to_string(),
            name: twentytwo::NAME.to_string(),
            part_one: twentytwo::part_one,
            part_two: twentytwo::part_two
        }),
        "23" => Some(Day {
            day: "23".to_string(),
            name: twentythree::NAME.to_string(),
            part_one: twentythree::part_one,
            part_two: twentythree::part_two
        }),
        "24" => Some(Day {
            day: "24".to_string(),
            name: twentyfour::NAME.to_string(),
            part_one: twentyfour::part_one,
            part_two: twentyfour::part_two
        }),
        "25" => Some(Day {
            day: "25".to_string(),
            name: twentyfive::NAME.to_string(),
            part_one: twentyfive::part_one,
            part_two: twentyfive::part_two
        }),
        _ => None
    }
}
