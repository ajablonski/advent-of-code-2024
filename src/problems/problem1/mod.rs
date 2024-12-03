use crate::Event;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use crate::problems::Problem;

#[derive(Clone)]
pub struct Problem1 {}

impl Problem<u128> for Problem1 {
    fn part1(&self, _input: &str, _tx: Sender<Event>) -> u128 {
        let (mut left_list, mut right_list): (Vec<u128>, Vec<u128>) = _input
            .lines()
            .map(|line| {
                let parts: Vec<u128> = line.split(" ").flat_map(|s| s.parse::<u128>()).collect();

                (parts[0], parts[1])
            })
            .unzip();

        left_list.sort();
        right_list.sort();

        left_list
            .into_iter()
            .zip(right_list)
            .fold(0, |acc, (a, b)| acc + b.abs_diff(a))
    }

    fn part2(&self, input: &str, _tx: Sender<Event>) -> u128 {
        let mut right_counts: HashMap<u128, u128> = HashMap::new();

        input
            .lines()
            .map(|line| {
                let parts: Vec<u128> = line.split(" ").flat_map(|s| s.parse::<u128>()).collect();

                right_counts
                    .entry(parts[1])
                    .and_modify(|c| *c += 1)
                    .or_insert(1);

                parts[0]
            })
            .collect::<Vec<u128>>()
            .into_iter()
            .fold(0u128, |acc, num| match right_counts.get(&num) {
                None => acc,
                Some(count) => acc + count * num,
            })
    }

}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use super::*;
    const P: Problem1 = Problem1 {};

    #[test]
    fn should_sort_and_sum_differences_when_0() {
        let sample_input = "\
        3 1\n\
        2 2\n\
        1 3";

        assert_eq!(P.part1(sample_input, mpsc::channel().0), 0);
    }

    #[test]
    fn should_find_when_numbers_are_different() {
        let sample_input = "\
        4 1\n\
        2 2\n\
        1 3";
        assert_eq!(P.part1(sample_input, mpsc::channel().0), 1);
    }

    #[test]
    fn should_should_calculate_similarity_score_for_part2_using_sample_data() {
        let sample_input = "\
        3   4\n\
        4   3\n\
        2   5\n\
        1   3\n\
        3   9\n\
        3   3";
        assert_eq!(P.part2(sample_input, mpsc::channel().0), 31);
    }

    #[test]
    fn should_calculate_similarity_score_for_very_similar_list() {
        let sample_input = "7  7";
        assert_eq!(P.part2(sample_input, mpsc::channel().0), 7);
    }

    #[test]
    fn should_ignore_elements_not_found_in_right_list() {
        let sample_input = "\
        7  7\n\
        2  6";
        assert_eq!(P.part2(sample_input, mpsc::channel().0), 7);
    }

    #[test]
    fn should_reflect_number_of_times_element_appears_in_right() {
        let sample_input = "\
        7  7\n\
        2  7";
        assert_eq!(P.part2(sample_input, mpsc::channel().0), 14);
    }

    #[test]
    fn should_calculate_similarity_score_for_very_similar_list_with_repeats() {
        let sample_input = "\
        7  7\n\
        7  7";
        assert_eq!(P.part2(sample_input, mpsc::channel().0), 7 * 2 * 2);
    }
}
