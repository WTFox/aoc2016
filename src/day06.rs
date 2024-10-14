pub fn part_one(input: &str) -> String {
    let mut result = String::new();

    let line_length = input.lines().next().unwrap().len();
    let mut lines: Vec<Vec<u8>> = vec![];
    for i in 0..line_length {
        for line in input.lines() {
            if line.len() != line_length {
                panic!("Line length mismatch");
            }
            let c = line.chars().nth(i).unwrap();
            let index = (c as u8 - b'a') as usize;
            if lines.len() <= i {
                lines.push(vec![0; 26]);
            }
            lines[i][index] += 1;
        }

        let mut max = 0;
        let mut max_index = 0;
        lines[i].iter().enumerate().for_each(|(index, &count)| {
            if count > max {
                max = count;
                max_index = index;
            }
        });

        result.push((max_index as u8 + b'a') as char);
    }

    result
}

pub fn part_two(input: &str) -> String {
    let mut result = String::new();

    let line_length = input.lines().next().unwrap().len();
    let mut lines: Vec<Vec<u8>> = vec![];
    for i in 0..line_length {
        for line in input.lines() {
            if line.len() != line_length {
                panic!("Line length mismatch");
            }
            let c = line.chars().nth(i).unwrap();
            let index = (c as u8 - b'a') as usize;
            if lines.len() <= i {
                lines.push(vec![0; 26]);
            }
            lines[i][index] += 1;
        }

        let mut count_so_far = lines[i][0];
        let mut least_common = 0;
        lines[i].iter().enumerate().for_each(|(index, &count)| {
            if count < count_so_far && count != 0 {
                count_so_far = count;
                least_common = index;
            }
        });

        result.push((least_common as u8 + b'a') as char);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#;

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one(INPUT), "easter");
    }

    #[test]
    fn test_part_one_input() {
        let input = include_str!("../inputs/day06.txt");
        assert_eq!(part_one(input), "qoclwvah");
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two(INPUT), "advent");
    }

    #[test]
    fn test_part_two_input() {
        let input = include_str!("../inputs/day06.txt");
        assert_eq!(part_two(input), "ryrgviuv");
    }
}
