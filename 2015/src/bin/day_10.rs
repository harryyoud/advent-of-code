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
    let mut chars = input.chars();

    let mut prev_char = chars.next().unwrap();
    let mut compressed_char_counts = vec![(prev_char, 1)];

    for c in chars {
        if prev_char == c {
            compressed_char_counts.last_mut().unwrap().1 += 1;
            continue;
        }
        compressed_char_counts.push((c, 1));
        prev_char = c;
    }

    let mut out = String::with_capacity(compressed_char_counts.len() * 2);
    for (c, amount) in compressed_char_counts {
        out.push_str(&amount.to_string());
        out.push(c);
    }

    out
}
