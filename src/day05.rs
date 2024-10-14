use md5::compute as md5_compute;

pub fn part_one(input: &str) -> String {
    let mut output = String::new();
    let input = input.trim();

    let mut i = 0u64;
    while output.len() < 8 {
        let hash_input = format!("{}{}", input, i);
        let hash = md5_compute(hash_input.as_bytes());
        if hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xF0) == 0 {
            let digit = format!("{:x}", hash[2] & 0x0F);
            output.push_str(&digit);
        }
        i += 1;
    }
    output
}

pub fn part_two(input: &str) -> String {
    let input = input.trim();

    let mut i = 0u64;
    let mut out: Vec<char> = vec!['-'; 8];
    let mut found = 0;

    while found < 8 {
        let hash_input = format!("{}{}", input, i);
        let hash = md5_compute(hash_input.as_bytes());

        if hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xF0) == 0 {
            let pos = (hash[2] & 0x0F) as usize;
            if pos < 8 && out[pos] == '-' {
                let val = format!("{:x}", (hash[3] & 0xF0) >> 4);
                out[pos] = val.chars().next().unwrap();
                found += 1;
            }
        }
        i += 1;
    }
    out.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = "abc";
        assert_eq!(part_one(input), "18f47a30");
    }
}
