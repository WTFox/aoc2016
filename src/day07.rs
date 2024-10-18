use itertools::Itertools;

pub fn part_one(input: &str) -> usize {
    process_input(input, supports_tls)
}

pub fn part_two(input: &str) -> usize {
    process_input(input, supports_ssl)
}

fn process_input<F>(input: &str, callback: F) -> usize
where
    F: Fn(&str) -> bool,
{
    input.trim().lines().map(callback).filter(|x| *x).count()
}

fn parse_sequences(input: &str) -> (Vec<String>, Vec<String>) {
    let mut inside_brackets = false;
    let (mut hypernets, mut supernets) = (vec![], vec![]);
    let mut current_sequence = String::from("");
    for c in input.chars() {
        match c {
            '[' => {
                inside_brackets = true;
                supernets.push(current_sequence.clone());
                current_sequence.clear();
            }
            ']' => {
                inside_brackets = false;
                hypernets.push(current_sequence.clone());
                current_sequence.clear();
            }
            _ => current_sequence.push(c),
        }
    }

    if !current_sequence.is_empty() {
        let seq = current_sequence.clone();
        if inside_brackets {
            hypernets.push(seq);
        } else {
            supernets.push(seq);
        }
    }

    (hypernets, supernets)
}

fn is_abba(haystack: &str) -> bool {
    for (a, b, c, d) in haystack.chars().tuple_windows() {
        if a != b && a == d && b == c {
            return true;
        }
    }
    false
}

fn supports_tls(addr: &str) -> bool {
    let (hypernets, supernets) = parse_sequences(addr);

    for seq in hypernets {
        if is_abba(&seq) {
            return false;
        }
    }

    for seq in supernets {
        if is_abba(&seq) {
            return true;
        }
    }

    false
}

fn supports_ssl(addr: &str) -> bool {
    let (hypernets, supernets) = parse_sequences(addr);
    let mut abas = vec![];
    for seq in supernets {
        for (a, b, c) in seq.chars().tuple_windows() {
            if a != b && a == c {
                abas.push(format!("{}{}{}", a, b, c));
            }
        }
    }
    for seq in hypernets {
        for aba in &abas {
            let bab = format!(
                "{}{}{}",
                aba.chars().nth(1).unwrap(),
                aba.chars().next().unwrap(),
                aba.chars().nth(1).unwrap()
            );
            if seq.contains(&bab) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "abba[mnop]qrst",
        "abcd[bddb]xyyx",
        "aaaa[qwer]tyui",
        "ioxxoj[asdfgh]zxcvbn",
    ];

    #[test]
    fn test_is_abba() {
        assert!(is_abba("abba"));
        assert!(is_abba("baab"));
        assert!(is_abba("bddb"));
        assert!(is_abba("ioxxoj"));
    }

    #[test]
    fn test_supports_tls() {
        let test_cases: Vec<(&str, bool)> = vec![
            ("abba[mnop]qrst", true),
            ("abcd[bddb]xyyx", false),
            ("aaaa[qwer]tyui", false),
            ("ioxxoj[asdfgh]zxcvbn", true),
        ];

        for (t, res) in test_cases {
            assert_eq!(supports_tls(t), res);
        }
    }

    #[test]
    fn part_one_with_input() {
        assert_eq!(part_one(&INPUT.join("\n")), 2);
    }

    #[test]
    fn test_parse_sequences() {
        assert_eq!(
            parse_sequences("abba[mnop]qrst"),
            (
                vec![String::from("mnop")],
                vec![String::from("abba"), String::from("qrst")]
            )
        );

        assert_eq!(
            parse_sequences("abba[mnop]qrst[wtf]"),
            (
                vec![String::from("mnop"), String::from("wtf")],
                vec![String::from("abba"), String::from("qrst")]
            )
        );
    }
}
