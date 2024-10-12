use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Room {
    name: String,
    sector_id: i32,
    checksum: String,
}

impl Room {
    fn is_real(&self) -> bool {
        self.decrypt() == self.checksum
    }

    fn decrypt(&self) -> String {
        let counts = self.name.chars().fold(HashMap::new(), |mut acc, c| {
            if c == '-' {
                return acc;
            }
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        let mut letters: Vec<(char, u32)> = counts.iter().map(|(&c, &count)| (c, count)).collect();
        letters.sort_by(|&(a, count_a), &(b, count_b)| count_b.cmp(&count_a).then(a.cmp(&b)));
        letters.iter().take(5).map(|&(c, _)| c).collect()
    }
}

fn parse_room(room: &str) -> Room {
    let (name, checksum) = room.split_at(room.len() - 7);
    let checksum = checksum.trim_start_matches("[").trim_end_matches("]");

    let (name, sector_id) = name.split_at(name.len() - 3);
    let name = name.trim_end_matches("-");

    Room {
        name: name.to_string(),
        sector_id: sector_id.parse().unwrap(),
        checksum: checksum.to_string(),
    }
}

fn shift_name(name: &str, shift: i32) -> String {
    name.chars()
        .map(|c| {
            if c == '-' {
                ' '
            } else {
                let c = c as i32 - 'a' as i32;
                let c = ((c + shift) % 26 + 26) % 26;
                (c + 'a' as i32) as u8 as char
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(parse_room)
        .filter(|room| room.is_real())
        .map(|room| room.sector_id)
        .collect::<Vec<i32>>()
        .into_iter()
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    let rooms = input.trim().lines().map(parse_room).collect::<Vec<Room>>();
    for room in rooms {
        let decrypted = shift_name(&room.name, room.sector_id);
        if decrypted == "northpole object storage" {
            return room.sector_id;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = [
            "aaaaa-bbb-z-y-x-123[abxyz]",
            "a-b-c-d-e-f-g-h-987[abcde]",
            "not-a-real-room-404[oarel]",
            "totally-real-room-200[decoy]",
        ];
        assert_eq!(part_one(&input.join("\n")), 1514);
    }

    #[test]
    fn test_part_one_input() {
        let input = include_str!("../inputs/day04.txt");
        assert_eq!(part_one(input), 278221);
    }

    #[test]
    fn test_part_two_input() {
        let input = include_str!("../inputs/day04.txt");
        assert_eq!(part_two(input), 267);
    }

    #[test]
    fn test_parse_room() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]";
        assert_eq!(
            parse_room(input),
            Room {
                name: "aaaaa-bbb-z-y-x".to_string(),
                sector_id: 123,
                checksum: "abxyz".to_string(),
            }
        );
    }

    #[test]
    fn test_is_real() {
        let room = Room {
            name: "aaaaabbbzyx".to_string(),
            sector_id: 123,
            checksum: "abxyz".to_string(),
        };
        assert!(room.is_real());

        let room = parse_room("not-a-real-room-404[oarel]");
        assert!(room.is_real());
    }

    #[test]
    fn test_shift_name() {
        assert_eq!(
            shift_name("qzmt-zixmtkozy-ivhz", 343),
            "very encrypted name"
        );
    }
}
