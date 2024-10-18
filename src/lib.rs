use std::fs;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub mod direction;
pub mod point;

pub use direction::Direction;
pub use point::Point;

pub fn read_input(day: u8) -> String {
    let filename = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}
