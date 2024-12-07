use crate::problems::problem7::Operator::{ADD, CONCAT, MULTIPLY};
use crate::problems::Problem;

pub struct Problem7 {}


#[derive(Clone)]
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
    next_operator_key: u64,
    operators: Vec<Operator>,
}


impl Iterator for Equation {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_operator_key == 0 {
            None
        } else {
            let mut current_key = self.next_operator_key;
            self.next_operator_key -= 1;
            let operator_type_count = self.operators.len() as u64;

            self.operands
                .iter()
                .copied()
                .reduce(|l, r| {
                    let result = self.operators[(current_key % operator_type_count) as usize].operate(l, r);
                    current_key /= operator_type_count;

                    result
                })
        }
    }
}

impl Equation {
    fn parse(line: &str, operators: &Vec<Operator>) -> Self {
        let (test_value_str, operands_str) = line.split_once(": ").unwrap();

        let operands: Vec<u64> = operands_str
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();

        Equation {
            test_value: test_value_str.parse().unwrap(),
            next_operator_key: (operators.len() as u64).pow(operands.len() as u32 - 1),
            operands,
            operators: (*operators).clone(),
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
    fn parse(input: &str, operators: &Vec<Operator>) -> Vec<Equation> {
        input
            .lines()
            .map(|line| Equation::parse(line, operators))
            .collect()
    }

    fn solve(&self, input: &str, operators: &Vec<Operator>) -> u128 {
        let mut equations = Problem7::parse(input, operators);
        equations
            .iter_mut()
            .filter_map(|eq| {
                let test_value = eq.test_value.clone();
                if eq.any(|r| r == test_value) {
                    Some(test_value)
                } else {
                    None
                }
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

    mod equation {
        use super::*;

        #[test]
        fn should_iterate_for_two_item_equation() {
            let mut e = Equation::parse("4: 1 3", &vec![ADD, MULTIPLY]);

            assert_eq!(e.next(), Some(4)); // state is 2 .  1 + 3 . Mod 2 == 0 --> +
            assert_eq!(e.next(), Some(3)); // state is 1 .  1 * 3 . Mod 2 == 1 --> *
            assert_eq!(e.next(), None);
        }

        #[test]
        fn should_iterate_for_two_item_equation_with_concat() {
            let mut e = Equation::parse("4: 1 3", &vec![ADD, MULTIPLY, CONCAT]);

            assert_eq!(e.next(), Some(4)); // state is 3 .  1 + 3 . Mod 3 == 0 --> +
            assert_eq!(e.next(), Some(13)); // state is 2 .  1 || 3 . Mod 3 == 2 --> *
            assert_eq!(e.next(), Some(3)); // state is 1 .  1 * 3 . Mod 3 == 1 --> *
            assert_eq!(e.next(), None);
        }

        #[test]
        fn should_iterate_for_three_item_equation() {
            let mut e = Equation::parse("4: 1 3 2", &vec![ADD, MULTIPLY]);

            assert_eq!(e.next(), Some(6)); // state is 100 . 1 + 3 + 2 . Mod 2 == 0 --> + ; /2, then mod 2 == 0 --> +
            assert_eq!(e.next(), Some(6)); // state is 011 . 1 * 3 * 2 . Mod 2 == 1 --> * ; /2, then mod 2 == 1 --> *
            assert_eq!(e.next(), Some(8)); // state is 010 . 1 + 3 * 2 . Mod 2 == 0 --> + ; /2, then mod 2 == 1 --> *
            assert_eq!(e.next(), Some(5)); // state is 001 . 1 * 3 + 2 . Mod 2 == 1 --> * ; /2, then mod 2 == 1 --> *
            assert_eq!(e.next(), None);
        }
    }
}
