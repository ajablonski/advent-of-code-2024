use crate::problems::Problem;

pub struct Problem6 {}

impl Problem<u128> for Problem6 {}

#[cfg(test)]
mod tests {
    use super::*;
    const P: Problem6 = Problem6 {};

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
