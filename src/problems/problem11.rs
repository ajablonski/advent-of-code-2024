use itertools::Itertools;
use crate::problems::Problem;

pub struct Problem11 {}

impl Problem<u128> for Problem11 {
    fn part1(&self, input: &str) -> u128 {
        let mut state = input.strip_suffix("\n").unwrap().split(" ")
            .flat_map(|x| x.parse::<u64>())
            .collect_vec();

        (0..25)
            .for_each(|_| state = state.evolve());

        state.len() as u128
    }

    fn part2(&self, input: &str) -> u128 {
        let mut state = input.strip_suffix("\n").unwrap().split(" ")
            .flat_map(|x| x.parse::<u64>())
            .collect_vec();

        (0..75)
            .for_each(|i| {
                println!("Stage {i}: Length = {}", state.len());
                state = state.evolve()
            });

        state.len() as u128
    }
}

trait Evolve {
    fn evolve(&self) -> Self;
}

impl Evolve for Vec<u64> {
    fn evolve(&self) -> Self {
        self.clone()
            .into_iter()
            .flat_map(|i| {
                if i == 0 {
                    vec![1]
                } else if i.ilog10() % 2 == 1 {
                    let num_as_string = i.to_string();

                    let (part1, part2) = num_as_string.split_at(num_as_string.len() / 2);
                    vec![part1.parse::<u64>().unwrap(), part2.parse::<u64>().unwrap()]
                } else {
                    vec![i * 2024]
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_produce_correct_answer_for_part_1() {
        let p = Problem11 {};
        assert_eq!(
            p.part1("125 17\n"),
            55312
        );
    }

    #[test]
    #[should_panic]
    fn should_produce_correct_answer_for_part_2() {
        let p = Problem11 {};
        assert_eq!(p.part2(""), 0);
    }
}
