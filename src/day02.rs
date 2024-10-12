use crate::{Direction, Point};
use std::collections::HashMap;

type CoordinateMap = HashMap<(i32, i32), &'static str>;

fn get_part_one_keypad() -> CoordinateMap {
    let mut keypad = CoordinateMap::new();
    keypad.insert((0, 0), "1");
    keypad.insert((1, 0), "2");
    keypad.insert((2, 0), "3");
    keypad.insert((0, 1), "4");
    keypad.insert((1, 1), "5");
    keypad.insert((2, 1), "6");
    keypad.insert((0, 2), "7");
    keypad.insert((1, 2), "8");
    keypad.insert((2, 2), "9");
    keypad
}

fn get_part_two_keypad() -> CoordinateMap {
    let mut keypad = CoordinateMap::new();
    keypad.insert((2, 0), "1");
    keypad.insert((1, 1), "2");
    keypad.insert((2, 1), "3");
    keypad.insert((3, 1), "4");
    keypad.insert((0, 2), "5");
    keypad.insert((1, 2), "6");
    keypad.insert((2, 2), "7");
    keypad.insert((3, 2), "8");
    keypad.insert((4, 2), "9");
    keypad.insert((1, 3), "A");
    keypad.insert((2, 3), "B");
    keypad.insert((3, 3), "C");
    keypad.insert((2, 4), "D");
    keypad
}

fn attempt_move(keypad: &CoordinateMap, start: Point, direction: Direction) -> Point {
    let orig = start;
    let new_point = orig.moved(direction);

    if keypad.contains_key(&(new_point.x, new_point.y)) {
        new_point
    } else {
        orig
    }
}

fn process_input(input: &str, keypad: &CoordinateMap, start: Point) -> String {
    let mut location = start;
    let mut code = Vec::new();
    for line in input.trim().lines() {
        for c in line.trim().chars() {
            let movement = Direction::try_from(c).unwrap();
            location = attempt_move(keypad, location, movement);
        }
        code.push(*keypad.get(&(location.x, location.y)).unwrap());
    }

    code.concat()
}

pub fn part_one(input: &str) -> String {
    let keypad = get_part_one_keypad();
    process_input(input, &keypad, Point { x: 1, y: 1 })
}

pub fn part_two(input: &str) -> String {
    let keypad = get_part_two_keypad();
    process_input(input, &keypad, Point { x: 0, y: 2 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        assert_eq!(part_one(input), "1985");
    }

    #[test]
    fn test_part_one_input() {
        let input = include_str!("../inputs/day02.txt");
        assert_eq!(part_one(input), "36629");
    }

    #[test]
    fn test_part_two_example() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        assert_eq!(part_two(input), "5DB3");
    }

    #[test]
    fn test_part_two_input() {
        let input = include_str!("../inputs/day02.txt");
        assert_eq!(part_two(input), "99C3D");
    }
}
