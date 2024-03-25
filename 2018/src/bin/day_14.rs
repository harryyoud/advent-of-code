use aoc_2018::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(14).trim().to_string();
    let recipe_worker = RecipeWorker {
        scoreboard: vec![3, 7],
        cursors: vec![0, 1],
    };

    dbg!(part_1(recipe_worker.clone(), &input));
    dbg!(part_2(recipe_worker.clone(), &input));
}

fn part_1(mut recipes: RecipeWorker, input: &str) -> String {
    recipes.run_until_n_recipes(input.parse().unwrap()).iter().join("")
}

fn part_2(mut recipes: RecipeWorker, input: &str) -> usize {
    recipes.run_until_pattern_found(&number_to_digits(input))
}

fn number_to_digits(number: &str) -> Vec<usize> {
    number.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec()
}


#[derive(Clone)]
struct RecipeWorker {
    scoreboard: Vec<usize>,
    cursors: Vec<usize>,
}

impl RecipeWorker {
    fn tick(&mut self) -> usize {
        let original_length = self.scoreboard.len();
        let score = self.cursors.iter().map(|x| self.scoreboard[*x]).sum::<usize>();
        let recipe_1 = score / 10;
        let recipe_2 = score % 10;

        if recipe_1 > 0 {
            self.scoreboard.push(recipe_1);
        }
        self.scoreboard.push(recipe_2);

        for cursor in self.cursors.iter_mut() {
            *cursor += 1 + self.scoreboard[*cursor];
            *cursor %= self.scoreboard.len();
        }

        // return how many new recipes we made
        self.scoreboard.len() - original_length
    }

    // returns the the 10 recipes after the index given (inclusive)
    fn run_until_n_recipes(&mut self, iterations: usize) -> [usize; 10] {
        loop {
            self.tick();
            if self.scoreboard.len() >= iterations + 10 {
                break;
            }
        }
    
        self.scoreboard[iterations..(iterations+10)].try_into().unwrap()
    }

    // returns first index of match, but will only pick
    // up changes made on ticks run inside the function
    fn run_until_pattern_found(&mut self, pattern: &[usize]) -> usize {
        loop {
            let recipes_made = self.tick();

            for offset in 0..recipes_made {
                // if we made 2 recipes we need to check the final 5, but also 5 offset backwards by 1 at the end
                // slightly over-engineered in case in the future we make 3 recipes in one tick
                if self.scoreboard.len() >= pattern.len() + offset &&
                    pattern == &self.scoreboard[(self.scoreboard.len() - pattern.len() - offset)..(self.scoreboard.len() - offset)]
                {
                    return self.scoreboard.len() - (pattern.len() + offset);
                }
            }
        }
    }
}