use std::{collections::HashMap, fmt};

use aoc_2017::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(13);
    let firewall = parse_input(&input);

    dbg!(part_1(firewall.clone()));
    dbg!(part_2(firewall.clone()));
}

fn part_1(mut firewall: Firewall) -> usize {
    firewall.run_to_completion(false);
    firewall.score
}

fn part_2(mut firewall: Firewall) -> usize {
    for i in 0.. {
        firewall.tick_layers();
        let mut firewall = firewall.clone();
        firewall.run_to_completion(true);
        if !firewall.caught {
            return i + 1;
        }
    }

    unreachable!("No solution found");
}

#[derive(Clone)]
struct Firewall {
    layers: Vec<Option<Layer>>,
    position: isize,
    score: usize,
    caught: bool,
}

impl Firewall {
    fn move_forwards(&mut self) -> TickResult {
        if self.position.unsigned_abs() >= self.layers.len() {
            return TickResult::Finished;
        }
        self.position += 1;

        if let Some(Some(x)) = self.layers.get(self.position.unsigned_abs()) {
            if x.position == 0 {
                self.score += self.position as usize * x.depth;
                return TickResult::Caught;
            }
        }

        TickResult::Ok
    }

    fn tick_layers(&mut self) {
        for (_layer_num, layer) in self.layers.iter_mut().enumerate() {
            if let Some(layer) = layer {
                layer.tick();
            }
        }
    }

    fn run_to_completion(&mut self, break_on_caught: bool) {
        loop {
            match self.move_forwards() {
                TickResult::Ok => {},
                TickResult::Finished => break,
                TickResult::Caught => {
                    self.caught = true;
                    if break_on_caught {
                        break;
                    }
                },
            };
            self.tick_layers();
        }
    }
}

enum TickResult {
    Ok,
    Finished,
    Caught,
}


#[derive(Debug, Clone)]
struct Layer {
    depth: usize,
    position: usize,
    travelling: Direction,
}

#[derive(Debug, Clone)]
enum Direction {
    Up, Down
}

impl Layer {
    fn tick(&mut self) {
        if self.depth == 1 {
            return;
        }
        match self.travelling {
            Direction::Up => {
                if self.position == 0 {
                    self.travelling = Direction::Down;
                    self.position += 1;
                    return;
                }
                self.position -= 1;
            },
            Direction::Down => {
                if self.position == self.depth - 1 {
                    self.travelling = Direction::Up;
                    self.position -= 1;
                    return;
                }
                self.position += 1;
            },
        }
    }
}

fn parse_input(input: &str) -> Firewall {
    let mut layer_map = HashMap::new();

    let mut max_layer_num = 0;

    for line in input.lines() {
        let (layer_num, depth) = line.split(": ").collect_tuple().unwrap();
        let (layer_num, depth) = (layer_num.parse::<usize>().unwrap(), depth.parse::<usize>().unwrap());
        max_layer_num = max_layer_num.max(layer_num);
        layer_map.insert(layer_num, depth);
    }

    let mut layers = vec![];
    for x in 0..=max_layer_num {
        match layer_map.get(&x) {
            Some(depth) => layers.push(Some(Layer { travelling: Direction::Down, depth: *depth, position: 0 })),
            None => layers.push(None),
        }
    }

    Firewall {
        layers,
        position: -1,
        score: 0,
        caught: false,
    }
}

impl fmt::Display for Firewall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;
        let max_depth = self.layers
            .iter()
            .filter_map(|x| x.as_ref().map(|a| a.depth))
            .max()
            .unwrap();

        for layer_num in 0..(self.layers.len()) {
            write!(f, " {layer_num:>2}  ")?
        }
        writeln!(f)?;
        for depth in 0..max_depth {
            for (layer_num, layer) in self.layers.iter().enumerate() {
                match layer {
                    Some(x) => {
                        if depth == x.position {
                            if self.position as usize == layer_num && depth == 0 {
                                write!(f, "(SS) ")?;
                            } else {
                                write!(f, "[SS] ")?
                            }
                        } else if depth >= x.depth {
                            if self.position as usize == layer_num && depth == 0 {
                                write!(f, "(  ) ")?
                            } else {
                                write!(f, "     ")?
                            }
                        } else {
                            if self.position as usize == layer_num && depth == 0 {
                                write!(f, "(  ) ")?
                            } else {
                                write!(f, "[  ] ")?
                            }
                        }
                    },
                    None => {
                        if depth == 0 {
                            if self.position as usize == layer_num {
                                write!(f, "(..) ")?
                            } else {
                                write!(f, ".... ")?
                            }
                        } else {
                            write!(f, "     ")?
                        }
                    },
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}
