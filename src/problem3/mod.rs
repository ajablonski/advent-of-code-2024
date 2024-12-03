use regex::Regex;
use crate::Problem;

pub struct Problem3 {}

impl Problem<u128> for Problem3 {
    fn part1(&self, input: &str) -> u128 {
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

    fn part2(&self, input: &str) -> u128 {
        0
    }
}

pub const PROBLEM3: Problem3 = Problem3 {};

#[cfg(test)]
mod tests {
    use super::*;
    const P: Problem3 = Problem3 {};

    #[test]
    fn should_solve_part_1_example() {
        assert_eq!(P.part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
    }

    #[test]
    #[should_panic]
    fn test_part2() {
        assert_eq!(P.part2(""), 0);
    }
}
