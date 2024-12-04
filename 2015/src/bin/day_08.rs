use aoc_2015::get_input;

fn main() {
    let input = get_input(8);

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(lengths)
        .map(|(mem_len, acc_len)| mem_len - acc_len)
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|s| re_escape_len(s) - s.len() as u32)
        .sum()
}

// returns (length_in_memory, length_after escapes)
fn lengths(line: &str) -> (u32, u32) {
    let mem_len = line.len() as u32;

    let mut chars = line[1..line.len() - 1].chars();
    let mut acc_len = 0_u32;

    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next().unwrap() {
                'x' => {
                    chars.next();
                    chars.next();
                    acc_len += 1;
                }
                '\\' | '"' => {
                    acc_len += 1;
                }
                _ => panic!("Invalid escape"),
            },
            _ => acc_len += 1,
        };
    }

    (mem_len, acc_len)
}

fn re_escape_len(line: &str) -> u32 {
    line.chars()
        .map(|c| match c {
            '"' | '\\' => 2,
            _ => 1,
        })
        .sum::<u32>()
        + 2
}
