use std::io::{self, Write};

use aoc_2016::get_input;

fn main() {
    let input = get_input(5);
    let input = input.trim();
    let (part_1, part_2) = solve(input);
    dbg!(part_1, part_2);
}

fn get_md5_suffix(input: &str, suffix: u64) -> md5::Digest {
    md5::compute(format!("{input}{suffix}"))
}

fn solve(input: &str) -> (String, String) {
    let mut part_1 = vec![];
    let mut part_2 = [None; 8];

    print!("part_1: ________ / part_2: ________");
    let _ = io::stdout().flush();

    for i in 0.. {
        if part_1.len() == 8 && part_2.iter().all(|x| x.is_some()) {
            break;
        }
        let md5 = get_md5_suffix(input, i);
        if md5[0..2] == [0u8; 2] && md5[2] < 0x10 {
            let sixth = md5[2] << 4 >> 4;
            let seventh = md5[3] >> 4;
            if part_1.len() < 8 {
                part_1.push(format!("{:x?}", md5[2] << 4 >> 4).chars().next().unwrap());
            }
            if (0..=8).contains(&sixth) && part_2[sixth as usize].is_none() {
                part_2[sixth as usize] = Some(format!("{:x?}", seventh).chars().next().unwrap());
            }
            print!(
                "\rpart_1: {:_<8} / part_2: {}",
                part_1.iter().collect::<String>(),
                part_2.iter().map(|x| x.unwrap_or('_')).collect::<String>()
            );
            let _ = io::stdout().flush();
        }
    }
    println!();
    (
        part_1.into_iter().collect(),
        part_2.into_iter().map(|x| x.unwrap()).collect(),
    )
}
