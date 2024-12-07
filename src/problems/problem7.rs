use crate::problems::problem7::Operator::{ADD, CONCAT, MULTIPLY};
use crate::problems::Problem;
use itertools::Itertools;
use std::iter::repeat_n;

pub struct Problem7 {}

enum Operator {
    ADD,
    MULTIPLY,
    CONCAT,
}

impl Operator {
    pub fn operate(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            ADD => lhs + rhs,
            MULTIPLY => lhs * rhs,
            CONCAT => lhs * 10u64.pow(rhs.ilog10() + 1) + rhs,
        }
    }
}

struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn parse(line: &str) -> Self {
        let (test_value_str, operands_str) = line.split_once(": ").unwrap();

        let operands: Vec<u64> = operands_str
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();

        Equation {
            test_value: test_value_str.parse().unwrap(),
            operands,
        }
    }
}

impl Problem<u128> for Problem7 {
    fn part1(&self, input: &str) -> u128 {
        self.solve(input, &vec![ADD, MULTIPLY])
    }

    fn part2(&self, input: &str) -> u128 {
        self.solve(input, &vec![ADD, MULTIPLY, CONCAT])
    }
}

impl Problem7 {
    fn parse(input: &str) -> Vec<Equation> {
        input.lines().map(|line| Equation::parse(line)).collect()
    }

    fn solve(&self, input: &str, operators: &Vec<Operator>) -> u128 {
        Problem7::parse(input)
            .iter()
            .filter_map(|eq| {
                repeat_n(operators, eq.operands.len() - 1)
                    .multi_cartesian_product()
                    .find_map(|operator_combination| {
                        let mut iter = operator_combination.iter();
                        eq.operands
                            .clone()
                            .into_iter()
                            .reduce(|x, y| iter.next().unwrap().operate(x, y))
                            .filter(|&r| r == eq.test_value)
                    })
            })
            .sum::<u64>() as u128
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_produce_correct_answer_for_part_1() {
        let p = Problem7 {};
        assert_eq!(
            p.part1(
                "\
                190: 10 19\n\
                3267: 81 40 27\n\
                83: 17 5\n\
                156: 15 6\n\
                7290: 6 8 6 15\n\
                161011: 16 10 13\n\
                192: 17 8 14\n\
                21037: 9 7 18 13\n\
                292: 11 6 16 20"
            ),
            3749
        );
    }

    #[test]
    fn should_produce_correct_answer_for_part_2() {
        let p = Problem7 {};
        assert_eq!(
            p.part2(
                "\
                190: 10 19\n\
                3267: 81 40 27\n\
                83: 17 5\n\
                156: 15 6\n\
                7290: 6 8 6 15\n\
                161011: 16 10 13\n\
                192: 17 8 14\n\
                21037: 9 7 18 13\n\
                292: 11 6 16 20"
            ),
            11387
        );
    }
}
