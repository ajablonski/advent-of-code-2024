use crate::problem1::PROBLEM1;
use crate::problem2::PROBLEM2;
use crate::problem3::PROBLEM3;
use crate::problem4::PROBLEM4;
use chrono::{TimeZone, Utc};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub trait Problem<T> {
    fn part1(&self, _input: &str) -> T {
        todo!()
    }

    fn part2(&self, _input: &str) -> T {
        todo!()
    }
}

pub mod problem1;
pub mod problem10;
pub mod problem11;
pub mod problem12;
pub mod problem13;
pub mod problem14;
pub mod problem15;
pub mod problem16;
pub mod problem17;
pub mod problem18;
pub mod problem19;
pub mod problem2;
pub mod problem20;
pub mod problem21;
pub mod problem22;
pub mod problem23;
pub mod problem24;
pub mod problem25;
pub mod problem3;
pub mod problem4;
pub mod problem5;
pub mod problem6;
pub mod problem7;
pub mod problem8;
pub mod problem9;

const MAX_PROBLEM: u32 = 25;

pub fn solve(problem_number: &u32) {
    let mut problem_lookup: HashMap<u32, &dyn Problem<_>> = HashMap::new();

    problem_lookup.insert(1, &PROBLEM1);
    problem_lookup.insert(2, &PROBLEM2);
    problem_lookup.insert(3, &PROBLEM3);
    problem_lookup.insert(4, &PROBLEM4);

    if let Some(problem) = problem_lookup.get(&problem_number) {
        let input = std::fs::read_to_string(format!("data/{problem_number}.txt")).unwrap();

        let part_1_result = (*problem).part1(input.as_str());
        let part_2_result = (*problem).part2(input.as_str());
        println!("Part 1: {part_1_result}");
        println!("Part 2: {part_2_result}");
    } else {
        println!("Problem {problem_number} not yet solved")
    }
}

pub fn fetch_data() {
    let session_token = std::fs::read_to_string("session_id_file.txt")
        .or(std::env::var("SESSION_TOKEN"))
        .unwrap();

    for problem_number in 1..=MAX_PROBLEM {
        let now = Utc::now();
        let problem_ready_time = Utc
            .with_ymd_and_hms(2024, 12, problem_number, 5, 0, 0)
            .unwrap();
        if now >= problem_ready_time {
            println!("Fetching data for problem {problem_number}...");
            if let Ok(body) = reqwest::blocking::Client::new()
                .get(format!(
                    "https://adventofcode.com/2024/day/{problem_number}/input"
                ))
                .header("Cookie", format!("session={session_token}"))
                .send()
            {
                let t = body.text();
                let file = File::create(format!("data/{problem_number}.txt"));

                match (t, file) {
                    (Ok(tt), Ok(mut f)) => {
                        let _ = f.write_all(tt.as_bytes());
                    }
                    (Err(e), _) => println!("Error in AOC Response: {e}"),
                    (Ok(_), Err(e)) => println!("Error in file opening: {e}"),
                }
            }
        } else {
            println!(
                "Data not yet available for problem {problem_number}. Current time is {now}, will be ready at {problem_ready_time}"
            )
        }
    }
}
