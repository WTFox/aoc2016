pub fn process_input<F>(input: &str, select_char: F) -> String
where
    F: Fn(&[usize; 26]) -> char,
{
    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    if lines.is_empty() {
        return String::new();
    }
    let line_length = lines[0].len();
    let mut result = String::with_capacity(line_length);

    for &line in &lines {
        if line.len() != line_length {
            panic!("Line length mismatch");
        }
    }

    for i in 0..line_length {
        let mut counts = [0usize; 26];
        for line in &lines {
            let c = line[i];
            if c.is_ascii_lowercase() {
                counts[(c - b'a') as usize] += 1;
            }
        }
        let selected_char = select_char(&counts);
        result.push(selected_char);
    }

    result
}

pub fn part_one(input: &str) -> String {
    process_input(input, |counts| {
        let (max_index, _) = counts
            .iter()
            .enumerate()
            .max_by_key(|&(_index, &count)| count)
            .unwrap();
        (b'a' + max_index as u8) as char
    })
}

pub fn part_two(input: &str) -> String {
    process_input(input, |counts| {
        let (min_index, _) = counts
            .iter()
            .enumerate()
            .filter(|&(_index, &count)| count > 0)
            .min_by_key(|&(_index, &count)| count)
            .unwrap();
        (b'a' + min_index as u8) as char
    })
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
        let input = include_str!("../../inputs/day06.txt");
        assert_eq!(part_one(input), "qoclwvah");
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two(INPUT), "advent");
    }

    #[test]
    fn test_part_two_input() {
        let input = include_str!("../../inputs/day06.txt");
        assert_eq!(part_two(input), "ryrgviuv");
    }
}
