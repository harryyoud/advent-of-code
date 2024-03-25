use aoc_2016::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(9);
    let input = input.trim();

    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(line: &str) -> usize {
    decompressed_line_len(line, false)
}

fn part_2(line: &str) -> usize {
    decompressed_line_len(line, true)
}

fn decompressed_line_len(line: &str, recurse: bool) -> usize {
    if !line.contains(['(', ')']) {
        return line.len();
    }
    let mut iter = line.chars().enumerate();
    let mut length = 0;

    loop {
        let Some((i, x)) = iter.next() else {
            break;
        };

        if x == '(' {
            let comp_instruction = iter.take_while_ref(|(_, c)| *c != ')').map(|(_, c)| c).collect::<String>();
            let (comp_len, comp_amount) = comp_instruction.split('x').map(|x| x.parse::<usize>().unwrap()).collect_tuple().unwrap();
            let _ = iter.next(); // )
            if recurse {
                let start = comp_instruction.len() + i + 2;
                length += decompressed_line_len(&line[start..(start + comp_len)], true) * comp_amount;
            } else {
                length += comp_len * comp_amount;
            }
            for _ in 0..comp_len {
                iter.next();
            }
            continue;
        }

        length += 1;
    }

    length
}