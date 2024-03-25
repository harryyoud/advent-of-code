use aoc_2016::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(18);
    let first_row = parse_input(input.trim());

    dbg!(part_1(&first_row));
    dbg!(part_2(&first_row));
}

fn part_1(first_row: &Vec<Tile>) -> usize {
    count_safe_tiles(first_row, 40)
}

fn part_2(first_row: &Vec<Tile>) -> usize {
    count_safe_tiles(first_row, 400_000)
}

fn count_safe_tiles(first_row: &Vec<Tile>, rows: usize) -> usize {
    let mut above = first_row.clone();
    let mut total_safe = above.iter().filter(|x| x.is_safe()).count();

    for _ in 1..rows {
        above = (0..above.len()).map(|col| {
                calculate_tile(
                    if col == 0 { None } else { above.get(col - 1) },
                    above[col],
                    above.get(col + 1),
                )
            }).collect_vec();
        total_safe += above.iter().filter(|x| x.is_safe()).count();
    }

    total_safe
}

fn parse_input(input: &str) -> Vec<Tile> {
    input.chars().map(Tile::from).collect_vec()
}

fn calculate_tile(
    top_left: Option<&Tile>,
    top_center: Tile,
    top_right: Option<&Tile>,
) -> Tile {
    let top_left = top_left.unwrap_or(&Tile::Safe);
    let top_right = top_right.unwrap_or(&Tile::Safe);
    match (top_left.is_trap(), top_center.is_trap(), top_right.is_trap()) {
        (true, true, false) => Tile::Trap,
        (false, true, true) => Tile::Trap,
        (true, false, false) => Tile::Trap,
        (false, false, true) => Tile::Trap,
        _ => Tile::Safe,
    }
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Safe, Trap
}

impl Tile {
    fn is_safe(&self) -> bool {
        matches!(self, Self::Safe)
    }

    fn is_trap(&self) -> bool {
        matches!(self, Self::Trap)
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => panic!("Invalid character for tile: {value}"),
        }
    }
}