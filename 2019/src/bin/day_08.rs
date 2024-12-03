use itertools::Itertools;
use aoc_2019::get_input;

const HEIGHT: usize = 6;
const WIDTH: usize = 25;

fn main() {
    let input = get_input(8);

    let layers = input
        .chars()
        .chunks(HEIGHT * WIDTH)
        .into_iter()
        .map(|x| x.collect_vec())
        .collect_vec();

    dbg!(part_1(&layers));
    part_2(&layers);
}

fn part_1(layers: &Vec<Vec<char>>) -> usize {
    let fewest_zeros_layer = layers.iter()
        .min_by_key(|layer| {
            layer.into_iter().filter(|x| **x == '0').count()
        })
        .unwrap();

    let counts = fewest_zeros_layer.into_iter().counts();
    counts.get(&'1').unwrap_or(&0) * counts.get(&'2').unwrap_or(&0)
}

fn part_2(layers: &Vec<Vec<char>>) {
    let canvas = collapse_layers(layers);
    print_canvas(&canvas);
}

fn collapse_layers(layers: &Vec<Vec<char>>) -> Vec<Colour> {
    let mut canvas = layers.last().unwrap().iter().map(|c| Colour::from_char(*c)).collect_vec();

    for layer in layers.iter().rev() {
        for (i, c) in layer.iter().enumerate() {
            use Colour::*;
            canvas[i] = match (canvas[i], Colour::from_char(*c)) {
                (_, colour @ Black) => colour,
                (_, colour @ White) => colour,
                (colour , Transparent) => colour,
            }
        }
    }

    canvas
}

fn print_canvas(canvas: &[Colour]) {
    for row in canvas.iter().chunks(WIDTH).into_iter() {
        println!("{}", row.map(|colour| colour.to_ascii()).join(""));
    }
}

#[derive(Copy, Clone, Debug)]
enum Colour {
    Black,
    White,
    Transparent,
}

impl Colour {
    fn from_char(input: char) -> Colour {
        use Colour::*;
        match input {
            '0' => Black,
            '1' => White,
            '2' => Transparent,
            x => panic!("Invalid colour code: {x}")
        }
    }

    fn to_ascii(&self) -> char {
        use Colour::*;
        match self {
            Black => ' ',
            White => '█',
            Transparent => ' ',
        }
    }
}
