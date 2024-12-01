use crate::Problem;

pub struct Problem2 {}

impl Problem for Problem2 {}

pub const PROBLEM2: Problem2 = Problem2 {};

#[cfg(test)]
mod tests {
    use super::*;
    const P: Problem2 = Problem2 {};

    #[test]
    fn test_part1() {
        assert_eq!(P.part1(""), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(P.part2(""), 0);
    }
}