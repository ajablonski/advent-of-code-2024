use crate::Problem;

pub struct Problem5 {}

impl Problem<u128> for Problem5 {}

pub const PROBLEM5: Problem5 = Problem5 {};

#[cfg(test)]
mod tests {
    use super::*;
    const P: Problem5 = Problem5 {};

    #[test]
    #[should_panic]
    fn test_part1() {
        assert_eq!(P.part1(""), 0);
    }

    #[test]
    #[should_panic]
    fn test_part2() {
        assert_eq!(P.part2(""), 0);
    }
}
