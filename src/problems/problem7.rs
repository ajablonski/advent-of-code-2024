use crate::problems::Problem;

pub struct Problem7 {}

impl Problem<u128> for Problem7 {}

#[cfg(test)]
mod tests {
    use super::*;
    const P: Problem7 = Problem7 {};

    #[test]
    #[should_panic]
    fn should_produce_correct_answer_for_part_1() {
        assert_eq!(P.part1(""), 0);
    }

    #[test]
    #[should_panic]
    fn should_produce_correct_answer_for_part_2() {
        assert_eq!(P.part2(""), 0);
    }
}
