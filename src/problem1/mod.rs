use crate::Problem;

pub struct Problem1 {}

impl Problem for Problem1 {}

#[cfg(test)]
mod tests {
    use super::*;
    const P: Problem1 = Problem1 {};

    #[test]
    fn test_part1() {
        assert_eq!(P.part1(""), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(P.part2(""), 0);
    }
}