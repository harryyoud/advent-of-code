use aoc_2015::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(16);

    let candidates = parse_input(&input);
    let template = Sue {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    dbg!(part_1(&candidates, &template));
    dbg!(part_2(&candidates, &template));
}

fn part_1(candidates: &[Sue], template: &Sue) -> u32 {
    candidates
        .iter()
        .enumerate()
        .filter(|(_idx, x)| x.is_match_exact(template))
        .map(|(idx, _x)| (idx + 1) as u32)
        .next()
        .unwrap()
}

fn part_2(candidates: &[Sue], template: &Sue) -> u32 {
    candidates
        .iter()
        .enumerate()
        .filter(|(_idx, x)| x.is_match_ranges(template))
        .map(|(idx, _x)| (idx + 1) as u32)
        .next()
        .unwrap()
}

#[derive(Default, Debug)]
struct Sue {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl Sue {
    fn is_match_exact(&self, template: &Sue) -> bool {
        self.children.map(|x| x == template.children.unwrap()).unwrap_or(true) &&
        self.cats.map(|x| x == template.cats.unwrap()).unwrap_or(true) &&
        self.samoyeds.map(|x| x == template.samoyeds.unwrap()).unwrap_or(true) &&
        self.pomeranians.map(|x| x == template.pomeranians.unwrap()).unwrap_or(true) &&
        self.akitas.map(|x| x == template.akitas.unwrap()).unwrap_or(true) &&
        self.vizslas.map(|x| x == template.vizslas.unwrap()).unwrap_or(true) &&
        self.goldfish.map(|x| x == template.goldfish.unwrap()).unwrap_or(true) &&
        self.trees.map(|x| x == template.trees.unwrap()).unwrap_or(true) &&
        self.cars.map(|x| x == template.cars.unwrap()).unwrap_or(true) &&
        self.perfumes.map(|x| x == template.perfumes.unwrap()).unwrap_or(true)
    }

    fn is_match_ranges(&self, template: &Sue) -> bool {
        self.children.map(|x| x == template.children.unwrap()).unwrap_or(true) &&
        self.cats.map(|x| x > template.cats.unwrap()).unwrap_or(true) &&
        self.samoyeds.map(|x| x == template.samoyeds.unwrap()).unwrap_or(true) &&
        self.pomeranians.map(|x| x < template.pomeranians.unwrap()).unwrap_or(true) &&
        self.akitas.map(|x| x == template.akitas.unwrap()).unwrap_or(true) &&
        self.vizslas.map(|x| x == template.vizslas.unwrap()).unwrap_or(true) &&
        self.goldfish.map(|x| x < template.goldfish.unwrap()).unwrap_or(true) &&
        self.trees.map(|x| x > template.trees.unwrap()).unwrap_or(true) &&
        self.cars.map(|x| x == template.cars.unwrap()).unwrap_or(true) &&
        self.perfumes.map(|x| x == template.perfumes.unwrap()).unwrap_or(true)
    }
}

fn parse_input(input: &str) -> Vec<Sue> {
    input.lines().map(parse_sue).collect_vec()
}

fn parse_sue(input: &str) -> Sue {
    let mut sue = Sue::default();

    let (_, sue_info) = input.split_once(": ").unwrap();

    for (property, amount) in sue_info.split(", ").map(|s| s.split(": ").collect_tuple().unwrap()) {
        let amount = Some(amount.parse().unwrap());

        match property {
            "children" => sue.children = amount,
            "cats" => sue.cats = amount,
            "samoyeds" => sue.samoyeds = amount,
            "pomeranians" => sue.pomeranians = amount,
            "akitas" => sue.akitas = amount,
            "vizslas" => sue.vizslas = amount,
            "goldfish" => sue.goldfish = amount,
            "trees" => sue.trees = amount,
            "cars" => sue.cars = amount,
            "perfumes" => sue.perfumes = amount,
            s => panic!("Invalid property: {s}")
        }
    }
    
    sue
}
