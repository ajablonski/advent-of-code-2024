use advent_of_code_2024::problem1::PROBLEM1;
use advent_of_code_2024::problem2::PROBLEM2;
use advent_of_code_2024::problem3::PROBLEM3;
use advent_of_code_2024::problem4::PROBLEM4;
use advent_of_code_2024::Problem;
use chrono::{TimeZone, Utc};
use clap::{Parser, Subcommand as ClapSubcommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug, ClapSubcommand)]
enum Subcommand {
    #[command()]
    Fetch,

    #[command(arg_required_else_help = true)]
    Solve { problem: u32 },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    #[command(subcommand)]
    subcommand: Subcommand,
}

const MAX_PROBLEM: u32 = 25;

fn main() {
    let args = Args::parse();
    let mut problem_lookup: HashMap<u32, &dyn Problem> = HashMap::new();

    problem_lookup.insert(1, &PROBLEM1);
    problem_lookup.insert(2, &PROBLEM2);
    problem_lookup.insert(3, &PROBLEM3);
    problem_lookup.insert(4, &PROBLEM4);

    match args.subcommand {
        Subcommand::Fetch => {
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
        Subcommand::Solve {
            problem: problem_number,
        } => {
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
    }
}
