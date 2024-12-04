use aoc_2016::get_input;
use itertools::Itertools;

const START: Point = (0, 0);
const END: Point = (3, 3);

type Point = (u32, u32);

fn main() {
    let input = get_input(17);
    let input = input.trim();

    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> String {
    let mut min_path_len = usize::MAX;
    let mut shortest_path = vec![];

    solve_with_dfs(
        input,
        &[],
        &START,
        &mut min_path_len,
        &mut shortest_path,
        SearchType::Shortest,
    );

    shortest_path
        .iter()
        .map(|x| x.as_char())
        .collect::<String>()
}

fn part_2(input: &str) -> usize {
    let mut max_path_len = usize::MIN;

    solve_with_dfs(
        input,
        &[],
        &START,
        &mut max_path_len,
        &mut vec![],
        SearchType::Longest,
    );

    max_path_len
}

fn solve_with_dfs(
    input: &str,
    path: &[Direction],
    current_point: &Point,
    len_result: &mut usize,
    path_result: &mut Vec<Direction>,
    search_type: SearchType,
) {
    let open_doors = get_open_doors(input, path);
    let possible_movements = possible_movements(&open_doors, current_point);

    if search_type.is_shortest() && path.len() > *len_result {
        return;
    }

    if *current_point == END {
        if search_type.is_shortest() {
            if *len_result > path.len() {
                *len_result = path.len();
                path.clone_into(path_result);
            }
        } else if *len_result < path.len() {
            // don't bother updating path_result as it isn't needed
            *len_result = path.len();
            path.clone_into(path_result);
        }
        return;
    }

    for (dir, next_point) in possible_movements {
        let mut new_path = path.to_owned();
        new_path.push(dir);
        solve_with_dfs(
            input,
            &new_path,
            &next_point,
            len_result,
            path_result,
            search_type,
        );
    }
}

fn calculate_hash(input: &str) -> String {
    format!("{:x}", md5::compute(input))
}

fn get_hash_with_path(input: &str, path: &[Direction]) -> String {
    let path_taken_hash_suffix = path.iter().map(|dir| dir.as_char()).collect::<String>();
    calculate_hash(&format!("{input}{path_taken_hash_suffix}"))
}

fn get_open_doors(input: &str, path: &[Direction]) -> Vec<Direction> {
    let hash = get_hash_with_path(input, path);
    hash.chars()
        .take(4)
        .zip(
            [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .iter(),
        )
        .filter_map(|(c, dir)| match c {
            'b' | 'c' | 'd' | 'e' | 'f' => Some(*dir),
            _ => None,
        })
        .collect_vec()
}

fn possible_movements(open_doors: &[Direction], current_point: &Point) -> Vec<(Direction, Point)> {
    let mut possible_directions = vec![];

    if current_point.0 > 0 && open_doors.contains(&Direction::Left) {
        possible_directions.push((Direction::Left, (current_point.0 - 1, current_point.1)));
    }
    if current_point.0 < END.0 && open_doors.contains(&Direction::Right) {
        possible_directions.push((Direction::Right, (current_point.0 + 1, current_point.1)));
    }
    if current_point.1 > 0 && open_doors.contains(&Direction::Up) {
        possible_directions.push((Direction::Up, (current_point.0, current_point.1 - 1)));
    }
    if current_point.1 < END.1 && open_doors.contains(&Direction::Down) {
        possible_directions.push((Direction::Down, (current_point.0, current_point.1 + 1)));
    }

    possible_directions
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_char(&self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }
}

#[derive(Clone, Copy)]
enum SearchType {
    Shortest,
    Longest,
}

impl SearchType {
    fn is_shortest(&self) -> bool {
        matches!(self, Self::Shortest)
    }
}
