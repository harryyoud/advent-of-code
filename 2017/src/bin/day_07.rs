use std::collections::{HashMap, HashSet};

use aoc_2017::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(7);
    let root_node = build_tree(&input);

    dbg!(part_1(&root_node));
    dbg!(part_2(&root_node));
}

fn part_1(root_node: &Node) -> &str {
    &root_node.name
}

fn part_2(root_node: &Node) -> usize {
    root_node.find_uneven_weight_recursive().unwrap()
}

#[derive(Debug)]
struct Node {
    name: String,
    weight: usize,
    children: Vec<Node>,
}

impl Node {
    fn find_uneven_weight_recursive(&self) -> Option<usize> {
        if self.are_children_balanced() {
            return None;
        }

        let counts = self.children.iter()
            .counts_by(|x| x.recursive_weight());
        let odd_one_out = counts.iter()
            .filter(|(_weight, count)| **count == 1)
            .map(|(weight, _count)| *weight)
            .next().unwrap();
        let others = counts.iter()
            .filter(|(_weight, count)| **count > 1)
            .map(|(weight, _count)| *weight)
            .next().unwrap();

        let uneven_child = self.children.iter()
            .find(|x| x.recursive_weight() == odd_one_out)
            .unwrap();

        match uneven_child.find_uneven_weight_recursive() {
            Some(x) => Some(x),
            None => Some(
                uneven_child.weight.checked_add_signed(others as isize - odd_one_out as isize).unwrap()
            ),
        }
    }

    fn are_children_balanced(&self) -> bool {
        self.children.iter().map(|x| x.recursive_weight()).all_equal()
    }

    fn recursive_weight(&self) -> usize {
        if self.children.is_empty() {
            return self.weight;
        }
        self.weight + self.children.iter().map(|x| x.recursive_weight()).sum::<usize>()
    }
}

fn build_tree(input: &str) -> Node {
    let mut nodes: HashMap::<&str, Node> = HashMap::new();
    let mut mapping: HashMap::<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (node_name, weight);
        let mut children = vec![];

        if line.contains(" -> ") {
            let (lhs, rhs) = line.split(" -> ").collect_tuple().unwrap();
            (node_name, weight) = parse_node(lhs);
            children.extend(rhs.split(", ").collect_vec());
        } else {
            (node_name, weight) = parse_node(line);
        }

        let node = Node {
            name: node_name.to_string(),
            weight,
            children: vec![],
        };
        mapping.insert(node_name, children);
        nodes.insert(node_name, node);
    }

    let root = find_root(&mapping);
    let children = mapping.get(root.as_str()).unwrap();
    let mut root = nodes.remove(root.as_str()).unwrap();
    recursively_add_children(&mut root, &mut nodes, children, &mapping);

    root
}

fn find_root(mapping: &HashMap<&str, Vec<&str>>) -> String {
    let parents = mapping.iter()
        .filter(|(_name, children)| !children.is_empty())
        .map(|(a, _b)| *a)
        .collect::<HashSet<_>>();
    let children = mapping.iter()
        .flat_map(|(_name, children)| children)
        .cloned()
        .collect::<HashSet<_>>();

    let mut root = parents.difference(&children).collect_vec();
    root.pop().unwrap().to_string()
}

fn recursively_add_children(
    root: &mut Node,
    nodes: &mut HashMap<&str, Node>,
    children: &[&str],
    mapping: &HashMap<&str, Vec<&str>>,
) {
    for child_name in children {
        let mut child = nodes.remove(*child_name).unwrap();
        recursively_add_children(&mut child, nodes, mapping.get(child_name).unwrap(), mapping);
        root.children.push(child);
    }
}

fn parse_node(input: &str) -> (&str, usize) {
    let (name, weight) = input.split_whitespace().collect_tuple().unwrap();
    let weight = weight.trim_matches(['(', ')']).parse().unwrap();

    (name, weight)
}