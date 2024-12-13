use crate::problems::Problem;

pub struct Problem13 {}

impl Problem<u128> for Problem13 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn should_produce_correct_answer_for_part_1() {
        let p = Problem13 {};
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
        let p = Problem13 {};
        assert_eq!(p.part2(""), 0);
    }
}
