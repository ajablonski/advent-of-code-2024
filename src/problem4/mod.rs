use crate::Problem;

pub struct Problem4 {}

impl Problem for Problem4 {}

pub const PROBLEM4: Problem4 = Problem4 {};

#[cfg(test)]
mod tests {
    use super::*;
    const P: Problem4 = Problem4 {};

    #[test]
    fn test_part1() {
        assert_eq!(P.part1(""), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(P.part2(""), 0);
    }
}
