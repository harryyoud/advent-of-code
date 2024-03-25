use aoc_2016::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(4);
    let rooms = parse_input(&input);
    dbg!(part_1(&rooms));
    dbg!(part_2(&rooms));
}

fn part_1(rooms: &[EncryptedRoom]) -> u32 {
    rooms
        .iter()
        .filter(|x| x.is_valid())
        .map(|x| x.sector_id)
        .sum()
}

fn part_2(rooms: &[EncryptedRoom]) -> u32 {
    rooms
        .iter()
        .filter(|x| x.is_valid())
        .filter(|x| x.decrypted_name() == "northpole object storage")
        .map(|x| x.sector_id)
        .next()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<EncryptedRoom> {
    let mut v = vec![];
    for line in input.lines() {
        let (room_name, sec_and_checksum) = line.rsplit_once('-').unwrap();
        let (sector_id, checksum) = sec_and_checksum.trim_end_matches(']').split_once('[').unwrap();
        v.push(EncryptedRoom {
            name: room_name.to_string(),
            sector_id: sector_id.parse().unwrap(),
            checksum: checksum.chars().collect_vec(),
        });
    }
    v
}

struct EncryptedRoom {
    name: String,
    sector_id: u32,
    checksum: Vec<char>,
}

impl EncryptedRoom {
    fn is_valid(&self) -> bool {
        let counts = self.name.chars().filter(|x| *x != '-').counts();
        let reverse_counts = counts
            .into_iter()
            .sorted_by_key(|(_char, count)| *count)
            .rev()
            .group_by(|(_char, count)| *count);

        let mut checksum = self.checksum.iter();

        for (_count, group) in reverse_counts.into_iter().take(5) {
            for (cha, _count) in group.into_iter().sorted() {
                let Some(checksum_next) = checksum.next() else {
                    break;
                };
                if cha != *checksum_next {
                    return false;
                }
            }
        }
        true
    }

    fn decrypted_name(&self) -> String {
        let shift = ('a'..='z').cycle();
        
        self.name
            .chars()
            .map(|c| {
                if c == '-' {
                    return ' ';
                }
                shift.clone().nth(self.sector_id as usize + c as usize - 'a' as usize).unwrap()
            })
            .collect()
    }
    
}