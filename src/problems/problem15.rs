use crate::problems::Problem;

pub struct Problem15 {}

impl Problem<u128> for Problem15 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn should_produce_correct_answer_for_part_1() {
        let p = Problem15 {};
        assert_eq!(
            p.part1(
                ""
            ),
            0
        );
    }

    #[test]
    #[should_panic]
    fn should_produce_correct_answer_for_part_2() {
        let p = Problem15 {};
        assert_eq!(p.part2(""), 0);
    }
}
