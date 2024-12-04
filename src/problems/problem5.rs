use crate::problems::Problem;

pub struct Problem5 {}

impl Problem<u128> for Problem5 {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    const P: Problem5 = Problem5 {};

    #[test]
    #[should_panic]
    fn should_produce_correct_answer_for_part_1() {
        assert_eq!(P.part1("", mpsc::channel().0), 0);
    }

    #[test]
    #[should_panic]
    fn should_produce_correct_answer_for_part_2() {
        assert_eq!(P.part2("", mpsc::channel().0), 0);
    }
}
