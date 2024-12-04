use aoc_2015::get_input;

fn main() {
    let input = get_input(4);
    let input = input.trim();

    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn get_md5_suffix(input: &str, suffix: u64) -> md5::Digest {
    md5::compute(format!("{input}{suffix}"))
}

fn part_1(input: &str) -> u64 {
    let mut out = 0;
    for i in 0.. {
        let md5 = get_md5_suffix(input, i);
        if md5[0..2] == [0u8; 2] && md5[2] < 0x10 {
            println!("md5({input}{i}) = {md5:x}");
            out = i;
            break;
        }
    }
    out
}

fn part_2(input: &str) -> u64 {
    let mut out = 0;
    for i in 0.. {
        let md5 = get_md5_suffix(input, i);
        if md5[0..3] == [0u8; 3] {
            println!("md5({input}{i}) = {md5:x}");
            out = i;
            break;
        }
    }
    out
}
