use crate::Problem;

pub struct Problem3 {}

impl Problem<u128> for Problem3 {}

pub const PROBLEM3: Problem3 = Problem3 {};

#[cfg(test)]
mod tests {
    use super::*;
    const P: Problem3 = Problem3 {};

    #[test]
    fn test_part1() {
        assert_eq!(P.part1(""), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(P.part2(""), 0);
    }
}
