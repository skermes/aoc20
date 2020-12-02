// See days/2.rs.
#![feature(str_split_once)]

#[macro_use]
extern crate lazy_static;

mod aoc_error;
mod days;

use std::io::Read;
use std::env;
use std::fmt::Display;
use std::time::{Instant, Duration};

use crate::aoc_error::AocError;
use crate::days::{get_day, Day};

fn format_result<V, E>(result: &Result<V, E>) -> String
    where V: Display,
          E: Display
{
    match result {
        Err(error) => format!("{}", error),
        Ok(solution) => format!("{}", solution)
    }
}

fn format_duration(duration: Duration) -> String {
    let millis = duration.as_millis();

    if millis == 0 {
        format!("{}\u{03BC}s", duration.as_micros())
    } else if millis > 1000 {
        let secs = duration.as_secs() as f64;
        let millis = millis as f64;
        format!("{:.1}s", secs + (millis / 1000.))
    } else {
        format!("{}ms", millis)
    }
}

struct DayReport {
    part_one_result: Result<String, AocError>,
    part_one_duration: Duration,
    part_two_result: Result<String, AocError>,
    part_two_duration: Duration
}

fn run(day: &Day) -> Result<DayReport, AocError> {
    let mut input_file = std::fs::File::open(format!("inputs/{}.txt", day.day))?;
    let mut buffer = String::new();
    input_file.read_to_string(&mut buffer)?;
    let input = buffer.trim();

    let start = Instant::now();
    let result_one = (day.part_one)(input);
    let duration_one = start.elapsed();

    let start = Instant::now();
    let result_two = (day.part_two)(input);
    let duration_two = start.elapsed();

    Ok(DayReport {
        part_one_result: result_one,
        part_one_duration: duration_one,
        part_two_result: result_two,
        part_two_duration: duration_two
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut days: Vec<Day> = Vec::new();
    if args.len() < 2 {
        for i in 1..26 {
            let day = get_day(&format!("{}", i));
            if day.is_some() {
                // Unwrap here won't panic because it's guarded by is_some.
                days.push(day.unwrap());
            }
        }
    } else {
        let day = get_day(&args[1]);

        if day.is_none() {
            println!("No implementation for day {}.", &args[1]);
            return;
        }

        // Unwrap here won't panic because we early return the None case above.
        days.push(day.unwrap());
    }

    let mut total_problem_duration = Duration::new(0, 0);
    let start = Instant::now();

    for day in days {
        let report = run(&day);

        println!("\nDay {}: {}", day.day, day.name);
        match report {
            Err(error) => println!("  {}", error),
            Ok(report) => {
                println!(
                    "  Part One: {:40} {}",
                    format_result(&report.part_one_result),
                    format_duration(report.part_one_duration)
                );
                println!(
                    "  Part Two: {:40} {}",
                    format_result(&report.part_two_result),
                    format_duration(report.part_two_duration)
                );
                total_problem_duration += report.part_one_duration + report.part_two_duration;
            }
        }
    }

    let total_duration = start.elapsed();
    println!(
        "\n Time - total: {}, problem: {}",
        format_duration(total_duration),
        format_duration(total_problem_duration)
    );
}
