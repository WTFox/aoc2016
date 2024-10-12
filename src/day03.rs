use itertools::Itertools;

pub fn is_triangle(sides: Vec<i32>) -> bool {
    let mut sides = sides;
    sides.sort();
    sides[0] + sides[1] > sides[2]
}

pub fn part_one(input: &str) -> i32 {
    let mut possibles = 0;
    for line in input.lines() {
        let sides: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if is_triangle(sides) {
            possibles += 1;
        }
    }
    possibles
}

pub fn part_two(input: &str) -> i32 {
    let mut possibles = 0;
    for chunk in &input.lines().chunks(3) {
        let mut nums = vec![];
        for line in chunk {
            let sides: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            nums.push(sides);
        }
        for i in 0..3 {
            if is_triangle(vec![nums[0][i], nums[1][i], nums[2][i]]) {
                possibles += 1;
            }
        }
    }
    possibles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_triangle() {
        let input = vec![5, 10, 25];
        assert!(!is_triangle(input))
    }

    #[test]
    fn test_part_one_example_one() {
        let input = " 5 10 25 \n";
        assert_eq!(part_one(input), 0);
    }

    #[test]
    fn test_part_one_input() {
        let input = include_str!("../inputs/day03.txt");
        assert_eq!(part_one(input), 1032);
    }
    #[test]
    fn test_part_two_input() {
        let input = include_str!("../inputs/day03.txt");
        assert_eq!(part_two(input), 1838);
    }
}
