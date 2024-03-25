use aoc_2015::get_input;

fn main() {
    let input = get_input(10);
    let input = input.trim();

    let mut part_1 = input.to_string();

    for _ in 0..40 {
        part_1 = transform(part_1);
    }

    dbg!(part_1.len());

    let mut part_2 = part_1;
    for _ in 40..50 {
        part_2 = transform(part_2);
    }

    dbg!(part_2.len());
}

fn transform(input: String) -> String {
    let mut chars = input.chars().peekable();

    let mut prev_char = chars.next().unwrap();
    let mut temp = vec![(prev_char, 1)];
    let mut cursor = 0;

    for c in chars {
        if prev_char == c {
            temp[cursor].1 += 1;
            continue;
        }
        temp.push((c, 1));
        cursor += 1;
        prev_char = c;
    }

    let mut out = String::new();
    for (c, amount) in temp {
        out.push_str(&amount.to_string());
        out.push(c);
    }

    out
}