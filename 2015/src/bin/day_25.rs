use aoc_2015::get_input;

fn main() {
    let input = get_input(25);
    let (row, column) = parse_input(&input);
    let code_nth = get_code_number_for(row, column);

    let mut out = 20151125_u64;
    for _ in 1..code_nth {
        out *= 252533;
        out %= 33554393;
    }
    dbg!(out);
}

fn get_code_number_for(row_aim: u32, col_aim: u32) -> u32 {
    let mut out = 1;
    let mut row = 1_u32;
    let mut col = 1_u32;
    while (row, col) != (row_aim, col_aim) {
        out += 1;
        if row == 1 {
            row = col + 1;
            col = 1;
            continue;
        }
        row -= 1;
        col += 1;
    }
    out
}

fn parse_input(input: &str) -> (u32, u32) {
    let start = input.find("row").unwrap();
    let (row, column) = input[start..].trim().trim_end_matches('.').split_once(", ").unwrap();
    let (_, row) = row.split_once(' ').unwrap();
    let (_, column) = column.split_once(' ').unwrap();
    (row.parse().unwrap(), column.parse().unwrap())
}