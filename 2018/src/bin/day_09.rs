#![feature(linked_list_cursors)]

use std::collections::{linked_list::CursorMut, HashMap, LinkedList};

use aoc_2018::get_input;
use lazy_regex::regex_captures;

fn main() {
    let input = get_input(9);
    let (player_count, last_marble) = parse_input(&input);

    dbg!(part_1(player_count, last_marble));
    dbg!(part_2(player_count, last_marble));
}

fn part_1(player_count: usize, last_marble: usize) -> usize {
    solve(player_count, last_marble)
}

fn part_2(player_count: usize, last_marble: usize) -> usize {
    solve(player_count, last_marble * 100)
}

fn solve(player_count: usize, last_marble: usize) -> usize {
    let mut circle = LinkedList::<usize>::from_iter([0]);
    let mut cursor = Circle {
        cursor: circle.cursor_front_mut(),
    };
    let mut current_player = 1;
    let mut player_scores = HashMap::<usize, usize>::new();

    for current_marble in 1..=last_marble {
        if current_marble % 23 == 0 {
            cursor.move_prev_n(7);
            let score = current_marble + cursor.pop_at_cursor();
            player_scores
                .entry(current_player)
                .and_modify(|x| *x += score)
                .or_insert(score);
        } else {
            cursor.move_next();
            cursor.insert_at_cursor(current_marble);
        }

        current_player = (current_player + 1) % player_count;
    }

    player_scores.into_values().max().unwrap()
}

/* We need to wrap CursorMut to ignore the "ghost element" */
struct Circle<'a> {
    cursor: CursorMut<'a, usize>,
}

impl Circle<'_> {
    fn move_prev(&mut self) {
        self.cursor.move_prev();
        if self.cursor.current().is_none() {
            self.cursor.move_prev();
        }
    }

    fn move_prev_n(&mut self, value: usize) {
        for _ in 0..value {
            self.move_prev();
        }
    }

    fn move_next(&mut self) {
        self.cursor.move_next();
        if self.cursor.current().is_none() {
            self.cursor.move_next();
        }
    }

    fn pop_at_cursor(&mut self) -> usize {
        let out = self.cursor.remove_current().unwrap();
        if self.cursor.current().is_none() {
            self.cursor.move_next();
        }
        out
    }

    fn insert_at_cursor(&mut self, value: usize) {
        match self.cursor.current() {
            Some(_) => self.cursor.insert_after(value),
            None => self.cursor.push_back(value),
        }
        self.cursor.move_next()
    }
}

fn parse_input(input: &str) -> (usize, usize) {
    // "435 players; last marble is worth 71184 points"
    let (_, player_count, last_marble) = regex_captures!(
        r#"^(\d+) players; last marble is worth (\d+) points$"#,
        input.trim()
    )
    .unwrap();
    (player_count.parse().unwrap(), last_marble.parse().unwrap())
}
