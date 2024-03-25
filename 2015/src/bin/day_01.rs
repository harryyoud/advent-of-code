use aoc_2015::get_input;

fn main() {
    let input = get_input(1);

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &str) -> i64 {
    let up = input.chars().filter(|x| *x == '(').count() as i64;
    let down = input.chars().filter(|x| *x == ')').count() as i64;

    up - down
}

fn part_2(input: &str) -> i64 {
    let mut floor = 0_i64;
    let mut out = 0_i64;
    for (pos, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("invalid character, pos {pos}")
        }
        if floor == -1 {
            out = pos as i64;
            break;
        }
    }
    out + 1
}