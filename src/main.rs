use clap::Parser;

use aoc2016::read_input;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    day: u8,
}

fn main() {
    let args = Args::parse();
    let day = args.day;

    match day {
        1 => {
            let input = read_input(day);
            println!("Part one: {}", aoc2016::day01::part_one(&input));
            println!("Part two: {}", aoc2016::day01::part_two(&input));
        }
        2 => {
            let input = read_input(day);
            println!("Part one: {}", aoc2016::day02::part_one(&input));
            println!("Part two: {}", aoc2016::day02::part_two(&input));
        }
        3 => {
            let input = read_input(day);
            println!("Part one: {}", aoc2016::day03::part_one(&input));
            println!("Part two: {}", aoc2016::day03::part_two(&input));
        }
        4 => {
            let input = read_input(day);
            println!("Part one: {}", aoc2016::day04::part_one(&input));
            println!("Part two: {}", aoc2016::day04::part_two(&input));
        }
        5 => {
            let input = read_input(day);
            println!("Part one: {}", aoc2016::day05::part_one(&input));
            println!("Part two: {}", aoc2016::day05::part_two(&input));
        }
        6 => {
            let input = read_input(day);
            println!("Part one: {}", aoc2016::day06::part_one(&input));
            println!("Part two: {}", aoc2016::day06::part_two(&input));
        }
        7 => {
            let input = read_input(day);
            println!("Part one: {}", aoc2016::day07::part_one(&input));
            println!("Part two: {}", aoc2016::day07::part_two(&input));
        }
        _ => println!("Day {} not implemented yet", day),
    }
}
