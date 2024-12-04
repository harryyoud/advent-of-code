use aoc_2020::get_input;
use itertools::Itertools;

// Input is a binary space partition where F means lower half and B means upper half
// Therefore, we can parse the row a binary number by replacing F with 0 and B with 1
// We do the same for the column with L=0, R=1
// The seat ID is ((row << 3) + column), and our input is **********, so we can parse each line as a
// binary number                                          <-row->^^^
//                                                               col
// We could prematurely optimise and use regex replace_all or similar, but this works quick enough
fn main() {
    let input = get_input(5);
    let input = input
        .replace('F', "0")
        .replace('B', "1")
        .replace('L', "0")
        .replace('R', "1");

    let mut seats = input.lines().map(binary_str_to_num).collect_vec();
    seats.sort();

    dbg!(part_1(&seats));
    dbg!(part_2(&seats));
}

fn part_1(seats: &[u32]) -> u32 {
    *seats.iter().max().unwrap()
}

// Assumes a sorted array, we can just step through and find the first number that is not contiguous
fn part_2(seats: &[u32]) -> u32 {
    for (before, after) in seats.iter().tuple_windows() {
        if *after != before + 1 {
            return before + 1;
        }
    }
    unreachable!("No solutions found!")
}

fn binary_str_to_num(input: &str) -> u32 {
    u32::from_str_radix(&input, 2).unwrap()
}
