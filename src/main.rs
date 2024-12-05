pub mod day_four;
pub mod day_one;
pub mod day_three;
pub mod day_two;
use clap::{Parser, Subcommand};
use std::fs;

#[derive(Debug, Subcommand)]
enum Day {
    One,
    Two,
    Three,
    Four,
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
        Some(Day::Three) => {
            day_three::solve(input);
        }
        Some(Day::Four) => {
            day_four::solve(input);
        }
        None => {}
    }
}
