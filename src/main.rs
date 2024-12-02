use clap::{Parser, Subcommand as ClapSubcommand};
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

    match args.subcommand {
        Subcommand::Fetch => advent_of_code_2024::fetch_data(),
        Subcommand::Solve {
            problem: problem_number,
        } => advent_of_code_2024::solve(&problem_number),
    }
}
