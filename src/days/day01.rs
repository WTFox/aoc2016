use crate::{Direction, Point};
use std::collections::HashSet;

fn gather_points_visited(input: &str) -> Vec<Point> {
    let mut point = Point::new();
    let mut direction = Direction::Up;
    let mut points_visited = Vec::new();

    for item in input.split(",") {
        let (turn, distance_str) = item.trim().split_at(1);
        direction = direction.turn(turn.parse::<char>().unwrap());
        for _ in 0..distance_str.parse::<i32>().unwrap() {
            point.move_in_direction(direction);
            points_visited.push(point);
        }
    }
    points_visited
}

pub fn part_one(input: &str) -> i32 {
    let points_visited = gather_points_visited(input);
    points_visited.last().unwrap().distance()
}

pub fn part_two(input: &str) -> i32 {
    let mut seen: HashSet<Point> = HashSet::new();
    let points_visited = gather_points_visited(input);

    for point in points_visited.iter() {
        if !seen.insert(*point) {
            return point.distance();
        }
        seen.insert(*point);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_one() {
        let input = "R2, L3";
        let result = part_one(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part_one_example_two() {
        let input = "R2, R2, R2";
        let result = part_one(input);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_part_one_example_three() {
        let input = "R5, L5, R5, R3";
        let result = part_one(input);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part_two() {
        let input = "R8, R4, R4, R8";
        let result = part_two(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_one_input() {
        let input = include_str!("../../inputs/day01.txt");
        let result = part_one(input);
        assert_eq!(result, 243);
    }

    #[test]
    fn test_part_two_input() {
        let input = include_str!("../../inputs/day01.txt");
        let result = part_two(input);
        assert_eq!(result, 142);
    }
}
