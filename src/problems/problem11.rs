use crate::problems::Problem;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Problem11 {}

impl Problem<u128> for Problem11 {
    fn part1(&self, input: &str) -> u128 {
        self.solve(input, 25)
    }

    fn part2(&self, input: &str) -> u128 {
        self.solve(input, 75)
    }
}

impl Problem11 {
    fn solve(&self, input: &str, iterations: usize) -> u128 {
        let mut state = input
            .strip_suffix("\n")
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .counts();

        (0..iterations).for_each(|_| state = state.evolve());

        state.values().sum::<usize>() as u128
    }
}

trait Evolve {
    fn evolve(&self) -> Self;
}

impl Evolve for HashMap<u64, usize> {
    fn evolve(&self) -> Self {
        let mut map = HashMap::new();

        self.into_iter().for_each(|(&index, count_appearances)| {
            if index == 0 {
                *map.entry(1).or_default() += count_appearances;
            } else if (index).ilog10() % 2 == 1 {
                let num_as_string = index.to_string();

                let (part1, part2) = num_as_string.split_at(num_as_string.len() / 2);

                *map.entry(part1.parse().unwrap()).or_default() += count_appearances;
                *map.entry(part2.parse().unwrap()).or_default() += count_appearances;
            } else {
                *map.entry(index * 2024).or_default() += count_appearances;
            }
        });

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_produce_correct_answer_for_part_1() {
        let p = Problem11 {};
        assert_eq!(p.part1("125 17\n"), 55312);
    }

    #[test]
    fn should_produce_correct_answer_for_part_2() {
        let p = Problem11 {};
        assert_eq!(p.part2("125 17\n"), 65601038650482);
    }
}
