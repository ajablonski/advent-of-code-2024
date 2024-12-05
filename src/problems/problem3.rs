use crate::display::AppDisplayState;
use crate::problems::problem3::ProgramState::{Disabled, Enabled};
use crate::problems::Problem;
use crate::Event;
use crate::Event::UpdateAppDisplayState;
use regex::Regex;
use std::sync::mpsc::Sender;

pub struct Problem3 {
    tx: Sender<Event>,
}

impl Problem<u128> for Problem3 {
    fn part1(&self, _input: &str) -> u128 {
        let instruction_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut partial_sum = 0u128;

        instruction_re
            .captures_iter(_input)
            .map(|c| match c.extract() {
                (_, [l, r]) => match (l.parse::<u32>(), r.parse::<u32>()) {
                    (Ok(l), Ok(r)) => {
                        partial_sum += (l * r) as u128;
                        self.tx
                            .send(UpdateAppDisplayState(AppDisplayState::part_1_only(
                                partial_sum,
                            )))
                            .unwrap_or_else(|e| println!("Error updating UI {:?}", e));
                        (l * r) as u128
                    }
                    _ => panic!("Unable to parse numbers {l:?}, {r:?}"),
                },
            })
            .sum()
    }

    fn part2(&self, _input: &str) -> u128 {
        let instruction_re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
        let mut partial_sum = 0u128;

        instruction_re
            .captures_iter(_input)
            .fold((Enabled, 0u128), |(program_state, total), c| {
                match (c.get(0).unwrap().as_str(), &program_state) {
                    ("do()", _) => (Enabled, total),
                    ("don't()", _) => (Disabled, total),
                    (_, Enabled) => {
                        match (
                            c.get(1).unwrap().as_str().parse::<u32>(),
                            c.get(2).unwrap().as_str().parse::<u32>(),
                        ) {
                            (Ok(l), Ok(r)) => {
                                partial_sum += (l * r) as u128;
                                self.tx
                                    .send(UpdateAppDisplayState(AppDisplayState::part_2_only(
                                        partial_sum,
                                    )))
                                    .unwrap_or_else(|e| println!("Error updating UI {:?}", e));


                                (program_state, total + (l * r) as u128)
                            }
                            _ => panic!("Unable to parse numbers {c:?}"),
                        }
                    }
                    (_, Disabled) => (program_state, total),
                }
            })
            .1
    }
}

impl Problem3 {
    pub(crate) fn new(tx: &Sender<Event>) -> Problem3 {
        Problem3 { tx: tx.clone() }
    }
}

enum ProgramState {
    Enabled,
    Disabled,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn should_solve_part_1_example() {
        let p: Problem3 = Problem3::new(&mpsc::channel().0);

        assert_eq!(
            p.part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn should_solve_part_2_example() {
        let p: Problem3 = Problem3::new(&mpsc::channel().0);

        assert_eq!(
            p.part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
