pub mod day_one;
pub mod day_two;
use clap::{Parser, Subcommand};
use std::fs;

#[derive(Debug, Subcommand)]
enum Day {
    One,
    Two,
}

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    day: Option<Day>,

    #[arg(short, long)]
    filename: String,
}

fn main() {
    let cli = Args::parse();
    let file = &cli.filename;
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    match &cli.day {
        Some(Day::One) => {
            day_one::solve(input);
        }
        Some(Day::Two) => {
            day_two::solve(input);
        }
        None => {}
    }
}
