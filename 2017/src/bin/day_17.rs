use std::collections::VecDeque;

use aoc_2017::get_input;

fn main() {
    let input = get_input(17);
    let input = input.parse::<usize>().unwrap();

    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: usize) -> usize {
    find_value_after_last_iter(input, 2017)
}

fn part_2(input: usize) -> usize {
    find_value_after_zero(input, 50_000_000)
}

fn find_value_after_last_iter(input: usize, iterations: usize) -> usize {
    let mut buffer = VecDeque::new();
    buffer.push_back(0);

    let mut cursor = 0;

    for i in 1..=iterations {
        cursor = (input + cursor) % buffer.len();
        buffer.insert(cursor + 1, i);
        cursor = (cursor + 1) % buffer.len();
    }

    buffer[(cursor + 1) % buffer.len()]
}

fn find_value_after_zero(input: usize, iterations: usize) -> usize {
    let mut buffer_len = 1;
    let mut cursor = 0;
    let mut position_0 = 0;
    let mut value_at_0_plus_1 = 0;

    for i in 1..=iterations {
        cursor = (input + cursor) % buffer_len;

        if (cursor + 1) <= position_0 {
            // value inserted before or at position of
            // value 0, so shift right
            position_0 += 1;
        }
        if (cursor + 1) == (position_0 + 1) {
            // writing to position right of zero,
            // so store this
            value_at_0_plus_1 = i;
        }

        buffer_len += 1;
        cursor = (cursor + 1) % buffer_len;
    }

    value_at_0_plus_1
}