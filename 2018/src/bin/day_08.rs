use aoc_2018::get_input;

fn main() {
    let input = get_input(8);
    let root = parse_input(&input);

    dbg!(part_1(&root));
    dbg!(part_2(&root));
}

fn part_1(root: &Node) -> usize {
    root.metadata_sum()
}

fn part_2(root: &Node) -> usize {
    root.value()
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn metadata_sum(&self) -> usize {
        self.children
            .iter()
            .map(|x| x.metadata_sum())
            .sum::<usize>()
            + self.metadata.iter().sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            return self.metadata.iter().sum::<usize>();
        }
        self.metadata
            .iter()
            .filter_map(|x| self.children.get(x - 1).map(|x| x.value()))
            .sum::<usize>()
    }
}

fn parse_input(input: &str) -> Node {
    let mut input = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    parse_node_recursive(&mut input)
}

fn parse_node_recursive(input: &mut impl Iterator<Item = usize>) -> Node {
    let child_count = input.next().expect("Early EOF when reading child_count");
    let metadata_count = input.next().expect("Early EOF when reading metadata_count");

    let mut node = Node {
        children: vec![],
        metadata: vec![],
    };

    for _ in 0..child_count {
        node.children.push(parse_node_recursive(input));
    }

    for _ in 0..metadata_count {
        node.metadata
            .push(input.next().expect("Early EOF when reading metadata"));
    }

    node
}
