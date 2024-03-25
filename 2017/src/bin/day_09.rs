use aoc_2017::get_input;

fn main() {
    let input = get_input(9);
    let (part_1, part_2) = run_to_completion(&input);

    dbg!(part_1, part_2);
}

fn run_to_completion(input: &str) -> (usize, usize) {
    let mut chars = input.trim().chars();
    let mut state = State {
        nested_group: 0,
        in_garbage: false,
        score: 0,
        garbage_count: 0,
    };

    while let Some(c) = chars.next() {
        match (c, state.in_garbage) {
            ('!', _) => {
                // skip next character
                let _ = chars.next();
            },
            ('<', false) => {
                state.in_garbage = true;
            },
            ('>', true) => {
                state.in_garbage = false;
            }
            ('}', false) => {
                state.nested_group -= 1;
            },
            ('{', false) => {
                state.nested_group += 1;
                state.score += state.nested_group;
            },
            (_, true) => {
                state.garbage_count += 1;
            },
            _ => {},
        }
    }

    (state.score, state.garbage_count)
}

#[derive(Debug)]
struct State {
    nested_group: usize,
    in_garbage: bool,
    garbage_count: usize,
    score: usize,
}