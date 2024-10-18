use anyhow::Result;
use clap::Parser;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
enum ParseError {
    #[error("Day not implemented: {0}")]
    NotImplemented(u8),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 1)]
    day: u8,
}

pub fn parse_args() -> Result<Args> {
    Ok(Args::try_parse()?)
}

fn read_input(day: u8) -> String {
    let filename = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

pub fn run(args: Args) -> Result<()> {
    let day = args.day;
    match day {
        1 => {
            let input = read_input(day);
            println!("Part one: {}", crate::day01::part_one(&input));
            println!("Part two: {}", crate::day01::part_two(&input));
            Ok(())
        }
        2 => {
            let input = read_input(day);
            println!("Part one: {}", crate::day02::part_one(&input));
            println!("Part two: {}", crate::day02::part_two(&input));
            Ok(())
        }
        3 => {
            let input = read_input(day);
            println!("Part one: {}", crate::day03::part_one(&input));
            println!("Part two: {}", crate::day03::part_two(&input));
            Ok(())
        }
        4 => {
            let input = read_input(day);
            println!("Part one: {}", crate::day04::part_one(&input));
            println!("Part two: {}", crate::day04::part_two(&input));
            Ok(())
        }
        5 => {
            let input = read_input(day);
            println!("Part one: {}", crate::day05::part_one(&input));
            println!("Part two: {}", crate::day05::part_two(&input));
            Ok(())
        }
        6 => {
            let input = read_input(day);
            println!("Part one: {}", crate::day06::part_one(&input));
            println!("Part two: {}", crate::day06::part_two(&input));
            Ok(())
        }
        7 => {
            let input = read_input(day);
            println!("Part one: {}", crate::day07::part_one(&input));
            println!("Part two: {}", crate::day07::part_two(&input));
            Ok(())
        }
        8 => {
            let input = read_input(day);
            println!("Part one: {}", crate::day08::part_one(&input));
            println!("Part two: {}", crate::day08::part_two(&input));
            Ok(())
        }
        _ => anyhow::bail!(ParseError::NotImplemented(day)),
    }
}
