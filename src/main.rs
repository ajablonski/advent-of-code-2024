use clap::{Parser, Subcommand as ClapSubcommand};
use std::error::Error;

#[derive(Debug, ClapSubcommand)]
enum Subcommand {
    #[command()]
    Fetch,

    #[command(arg_required_else_help = true)]
    Solve { problem: usize },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    #[command(subcommand)]
    subcommand: Subcommand,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.subcommand {
        Subcommand::Fetch => Ok(advent_of_code_2024::fetch_data()),
        Subcommand::Solve {
            problem: problem_number,
        } => advent_of_code_2024::solve(problem_number),
    }
}
