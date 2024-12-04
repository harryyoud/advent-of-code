use aoc_2020::get_input;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    let input = input.replace("(", "( ").replace(")", " )");
    let input = input
        .lines()
        .map(|x| x.split_whitespace())
        .map(|mut line| {
            let mut out = vec![];
            parse_recursive(&mut line, &mut out);
            out
        })
        .collect_vec();

    dbg!(part_1(&input));
}

fn part_1<'a>(input: &[Vec<Node>]) -> u64 {
    input
        .into_iter()
        .map(|nodes| solve_recursive(&mut nodes.into_iter()))
        .sum::<u64>()
}

fn solve_recursive<'a>(iter: &mut impl Iterator<Item = &'a Node>) -> u64 {
    let mut left = match iter.next().unwrap() {
        Node::Literal(x) => *x,
        Node::Unary(_) => panic!(),
        Node::Expression(x) => solve_recursive(&mut x.into_iter()),
    };

    while let Some(Node::Unary(operator)) = iter.next() {
        let right = iter.next().unwrap();

        use Node::*;
        left = match right {
            Literal(y) => operator.apply(left, *y),
            Expression(y) => operator.apply(left, solve_recursive(&mut y.into_iter())),
            _ => panic!("Invalid combination!"),
        };
    }

    left
}

fn parse_recursive<'a>(
    iter: &mut impl Iterator<Item = &'a str>,
    current_expression: &mut Vec<Node>,
) {
    while let Some(n) = iter.next() {
        current_expression.push(match n.chars().next().unwrap() {
            '(' => {
                let mut nodes = vec![];
                parse_recursive(iter, &mut nodes);
                Node::Expression(nodes)
            }
            '0'..='9' => Node::Literal(n.parse().unwrap()),
            '+' => Node::Unary(Operator::Add),
            '*' => Node::Unary(Operator::Multiply),
            ')' => return,
            _ => panic!("Invalid node: {n}"),
        });
    }
}

#[derive(Clone, Debug)]
enum Node {
    Literal(u64),
    Unary(Operator),
    Expression(Vec<Node>),
}

impl Node {
    fn as_literal(&self) -> Option<u64> {
        match self {
            Self::Literal(x) => Some(*x),
            _ => None,
        }
    }
    fn as_operator(&self) -> Option<Operator> {
        match self {
            Self::Unary(x) => Some(*x),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Operator {
    Multiply,
    Add,
}

impl Operator {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Operator::Multiply => left * right,
            Operator::Add => left + right,
        }
    }
}
