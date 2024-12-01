use advent_of_code_2024::{problem1::Problem1, Problem};
use clap::{Parser, Subcommand as ClapSubcommand};
use std::collections::HashMap;

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

fn main() {
    let args = Args::parse();
    let mut problem_lookup: HashMap<u32, &dyn Problem> = HashMap::new();

    problem_lookup.insert(1, &Problem1 {});

    match args.subcommand {
        Subcommand::Fetch => {
            println!("Fetching...")
        }
        Subcommand::Solve { problem: problem_number } => {
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
