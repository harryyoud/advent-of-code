use aoc_2016::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(2);
    let movements = parse_input(&input);

    dbg!(part_1(&movements));
    dbg!(part_2(&movements));
}

fn part_1(movements: &Vec<Vec<Direction>>) -> String {
    let mut keypad = Keypad {
        cursor: '5',
        columns: vec![
            vec!['1', '4', '7'],
            vec!['2', '5', '8'],
            vec!['3', '6', '9'],
        ],
        rows: vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ],
    };

    let mut code = vec![];

    for movement in movements {
        for direction in movement {
            keypad.move_dir(direction);
        }
        code.push(keypad.cursor);
    }

    code.into_iter().collect()
}

fn part_2(movements: &Vec<Vec<Direction>>) -> String {
    let mut keypad = Keypad {
        cursor: '5',
        columns: vec![
            vec!['5'],
            vec!['2', '6', 'A'],
            vec!['1', '3', '7', 'B', 'D'],
            vec!['4', '8', 'C'],
            vec!['9'],
        ],
        rows: vec![
            vec!['1'],
            vec!['2', '3', '4'],
            vec!['5', '6', '7', '8', '9'],
            vec!['A', 'B', 'C'],
            vec!['D'],
        ],
    };

    let mut code = vec![];

    for movement in movements {
        for direction in movement {
            keypad.move_dir(direction);
        }
        code.push(keypad.cursor);
    }

    code.into_iter().collect()
}

fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| match c {
                    'D' => Direction::Down,
                    'U' => Direction::Up,
                    'R' => Direction::Right,
                    'L' => Direction::Left,
                    s => panic!("Invalid direction: {s}"),
                })
                .collect_vec()
        })
        .collect_vec()
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

struct Keypad {
    cursor: char,
    columns: Vec<Vec<char>>,
    rows: Vec<Vec<char>>,
}

impl Keypad {
    fn move_dir(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.cursor = self.move_up(),
            Direction::Left => self.cursor = self.move_left(),
            Direction::Down => self.cursor = self.move_down(),
            Direction::Right => self.cursor = self.move_right(),
        }
    }

    fn move_up(&self) -> char {
        let column = self
            .columns
            .iter()
            .find(|a| a.iter().any(|b| *b == self.cursor))
            .unwrap();
        let pos = column.iter().position(|x| *x == self.cursor).unwrap();
        column[pos.saturating_sub(1)]
    }

    fn move_left(&self) -> char {
        let row = self
            .rows
            .iter()
            .find(|a| a.iter().any(|b| *b == self.cursor))
            .unwrap();
        let pos = row.iter().position(|x| *x == self.cursor).unwrap();
        row[pos.saturating_sub(1)]
    }

    fn move_down(&self) -> char {
        let column = self
            .columns
            .iter()
            .find(|a| a.iter().any(|b| *b == self.cursor))
            .unwrap();
        let pos = column.iter().position(|x| *x == self.cursor).unwrap();
        column[(pos + 1).min(column.len() - 1)]
    }

    fn move_right(&self) -> char {
        let row = self
            .rows
            .iter()
            .find(|a| a.iter().any(|b| *b == self.cursor))
            .unwrap();
        let pos = row.iter().position(|x| *x == self.cursor).unwrap();
        row[(pos + 1).min(row.len() - 1)]
    }
}
