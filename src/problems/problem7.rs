use crate::problems::Problem;

pub struct Problem7 {}

struct Equation {
    test_value: u64,
    operands: Vec<u64>,
    next_operator_key: u64
}

impl Iterator for Equation {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_operator_key == 0 {
            None
        } else {
            let mut current_key = self.next_operator_key.clone();
            self.next_operator_key -= 1;

            self.operands.iter().copied().into_iter().reduce(|l,  r| {
                if (current_key % 2) == 0 {
                    current_key /= 2;
                    l + r
                } else {
                    current_key /= 2;
                    l * r
                }
            }).map(|l| l.clone())
        }
    }
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
            next_operator_key: 2u64.pow(operands.len() as u32 - 1),
            operands,
        }
    }

}

impl Problem<u128> for Problem7 {
    fn part1(&self, input: &str) -> u128 {
        let mut equations = Problem7::parse(input);
        equations
            .iter_mut()
            .filter_map(|eq| {
                let test_value = eq.test_value.clone();
                if eq.any(|r| r == test_value) {
                    Some(test_value as u128)
                } else {
                    None
                }
            })
            .sum()
    }

    fn part2(&self, _input: &str) -> u128 {
        0
    }
}

impl Problem7 {
    fn parse(input: &str) -> Vec<Equation> {
        input.lines().map(|line| Equation::parse(line)).collect()
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
        assert_eq!(p.part2(""), 0);
    }

    mod equation {
        use super::*;

        #[test]
        fn should_iterate_for_two_item_equation() {
            let mut e = Equation::parse("4: 1 3");

            assert_eq!(e.next(), Some(4)); // state is 2 .  1 + 3 . Mod 2 == 0 --> +
            assert_eq!(e.next(), Some(3)); // state is 1 .  1 * 3 . Mod 2 == 1 --> *
            assert_eq!(e.next(), None);
        }

        #[test]
        fn should_iterate_for_three_item_equation() {
            let mut e = Equation::parse("4: 1 3 2");

            assert_eq!(e.next(), Some(6)); // state is 100 . 1 + 3 + 2 . Mod 2 == 0 --> + ; /2, then mod 2 == 0 --> +
            assert_eq!(e.next(), Some(6)); // state is 011 . 1 * 3 * 2 . Mod 2 == 1 --> * ; /2, then mod 2 == 1 --> *
            assert_eq!(e.next(), Some(8)); // state is 010 . 1 + 3 * 2 . Mod 2 == 0 --> + ; /2, then mod 2 == 1 --> *
            assert_eq!(e.next(), Some(5)); // state is 001 . 1 * 3 + 2 . Mod 2 == 1 --> * ; /2, then mod 2 == 1 --> *
            assert_eq!(e.next(), None);
        }
    }
}
