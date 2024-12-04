use aoc_2015::get_input;
use aoc_lib::grid::Grid;
use aoc_lib::vector::Vector;

fn main() {
    let input = get_input(18);
    let grid = parse_input(&input);

    dbg!(part_1(&grid));
    dbg!(part_2(&grid));
}

fn part_1(grid: &Grid<bool>) -> usize {
    let mut grid = grid.clone();
    for _ in 0..100 {
        tick(&mut grid);
    }
    count_on(&grid)
}

fn part_2(grid: &Grid<bool>) -> usize {
    let mut grid = grid.clone();
    for _ in 0..100 {
        tick(&mut grid);
        grid.insert(Vector::new([0, 0]), true);
        grid.insert(Vector::new([99, 0]), true);
        grid.insert(Vector::new([0, 99]), true);
        grid.insert(Vector::new([99, 99]), true);
    }
    count_on(&grid)
}

fn tick(grid: &mut Grid<bool>) {
    let previous = grid.clone();
    grid.tick(
        |point, value| match (value, neighbours_on(&previous, point)) {
            (true, 2..=3) => true,
            (false, 3) => true,
            _ => false,
        },
    )
}

fn count_on(grid: &Grid<bool>) -> usize {
    grid.values().copied().filter(|x| *x).count()
}

fn neighbours_on(grid: &Grid<bool>, point: Vector<2>) -> usize {
    point
        .neighbours_diagonals()
        .filter(|x| *grid.get(*x).unwrap_or(&false))
        .count()
}

fn parse_input(input: &str) -> Grid<bool> {
    Grid::parse(input, |c| match c {
        '#' => true,
        '.' => false,
        _ => panic!("Invalid character: {c}"),
    })
}
