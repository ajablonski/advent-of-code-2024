use std::sync::mpsc::Sender;
use regex::Regex;
use crate::Event;
use crate::problems::problem3::ProgramState::{Disabled, Enabled};
use crate::problems::Problem;

#[derive(Clone)]
pub struct Problem3 {}

impl Problem<u128> for Problem3 {

    fn part1(&self, input: &str, _tx: Sender<Event>) -> u128 {
        let instruction_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        instruction_re
            .captures_iter(input)
            .map(|c| {
                match c.extract() {
                    (_, [l , r]) => {
                        match (l.parse::<u32>(), r.parse::<u32>()) {
                            (Ok(l), Ok(r)) => (l * r) as u128,
                            _ => panic!("Unable to parse numbers {l:?}, {r:?}")
                        }
                    },
                }
            })
            .sum()
    }

    fn part2(&self, input: &str, _tx: Sender<Event>) -> u128 {
        let instruction_re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

        instruction_re
            .captures_iter(input)
            .fold((Enabled, 0u128), |(program_state, total), c| {
                match (c.get(0).unwrap().as_str(), &program_state) {
                    ("do()", _) => {
                        (Enabled, total)
                    },
                    ("don't()", _) => {
                        (Disabled, total)
                    },
                    (_, Enabled) => {
                        match (c.get(1).unwrap().as_str().parse::<u32>(), c.get(2).unwrap().as_str().parse::<u32>()) {
                            (Ok(l), Ok(r)) => (program_state, total + (l * r) as u128),
                            _ => panic!("Unable to parse numbers {c:?}")
                        }
                    },
                    (_, Disabled) => (program_state, total)
                }
            }).1
    }
}

enum ProgramState {
    Enabled,
    Disabled
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use super::*;
    const P: Problem3 = Problem3 {};

    #[test]
    fn should_solve_part_1_example() {
        assert_eq!(P.part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))", mpsc::channel().0), 161);
    }

    #[test]
    fn should_solve_part_2_example() {
        assert_eq!(P.part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", mpsc::channel().0), 48);
    }
}
