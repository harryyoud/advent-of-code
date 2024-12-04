use aoc_2024::get_input;
use aoc_lib::grid::Grid;
use aoc_lib::vector::Vector;

fn main() {
    let input = get_input(4);
    let grid = Grid::parse(&input, |c| c);
    let word_search = WordSearch { grid };

    dbg!(part_1(&word_search));
    dbg!(part_2(&word_search));
}

fn part_1(word_search: &WordSearch) -> usize {
    let word = "XMAS";
    word_search
        .grid
        .keys()
        .map(|point| word_search.word_starts_here_any_direction(word, point))
        .sum::<usize>()
}

fn part_2(word_search: &WordSearch) -> usize {
    let word = "MAS";
    word_search
        .grid
        .keys()
        .map(|point| word_search.criss_cross_middle(word, point))
        .filter(|x| *x)
        .count()
}

struct WordSearch {
    grid: Grid<char>,
}

impl WordSearch {
    // strategy: travel in all directions and fail early if doesn't line up with intended word
    fn word_starts_here_any_direction(&self, word: &str, point: Vector<2>) -> usize {
        Self::iter_in_all_directions(point, 0, word.len() as i32)
            .map(move |line| self.check_word(line, word))
            .filter(|x| *x)
            .count()
    }

    // strategy: iterate across both diagonals in both directions. If we get 2 matches (or 4 matches
    //           for a palindrome), then we have a criss-cross
    fn criss_cross_middle(&self, word: &str, center_point: Vector<2>) -> bool {
        assert_eq!(
            word.len() % 2,
            1,
            "Word must be an odd length to have a center letter"
        );
        let half_word_length = (word.len() / 2) as i32;

        [2, 4].contains(
            &Self::iter_across_diagonals(center_point, half_word_length)
                .map(move |diagonal| self.check_word(diagonal, word))
                .filter(|x| *x)
                .count(),
        )
    }

    fn check_word(&self, points: impl Iterator<Item = Vector<2>>, word: &str) -> bool {
        points.zip(word.chars()).all(|(point, word_char)| {
            self.grid
                .get(point)
                .map(|c| *c == word_char)
                .unwrap_or(false)
        })
    }

    // with middle of iter being the centre point specified
    fn iter_across_diagonals(
        center_point: Vector<2>,
        max_in_one_direction: i32,
    ) -> impl Iterator<Item = impl Iterator<Item = Vector<2>>> {
        [
            Vector::new([1, 1]),
            Vector::new([-1, 1]),
            Vector::new([1, -1]),
            Vector::new([-1, -1]),
        ]
        .into_iter()
        .map(move |direction| {
            ((-max_in_one_direction)..=max_in_one_direction)
                .map(move |factor| center_point + (direction * factor))
        })
    }

    fn iter_in_all_directions(
        point: Vector<2>,
        from: i32,
        to: i32,
    ) -> impl Iterator<Item = impl Iterator<Item = Vector<2>>> {
        Vector::new([0; 2])
            .neighbours_diagonals()
            .map(move |neighbour| {
                (from..to)
                    .map(move |factor| neighbour * factor)
                    .map(move |offset| point + offset)
            })
    }
}
