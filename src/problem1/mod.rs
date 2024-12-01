use crate::Problem;
use std::collections::HashMap;

pub struct Problem1 {}

impl Problem for Problem1 {
    fn part1(&self, input: &str) -> u128 {
        let mut left_list = Vec::new();
        let mut right_list = Vec::new();

        input.lines().for_each(|line| {
            let parts: Vec<&str> = line.split(" ").filter(|s| !s.is_empty()).collect();

            left_list.push(parts[0].parse::<u128>().unwrap());
            right_list.push(parts[1].parse::<u128>().unwrap());
        });

        left_list.sort();
        right_list.sort();

        left_list
            .iter()
            .zip(right_list)
            .fold(0, |acc, (a, b)| acc + b.abs_diff(*a))
    }

    fn part2(&self, input: &str) -> u128 {
        let mut left_list: Vec<u128> = Vec::new();
        let mut right_counts: HashMap<u128, u128> = HashMap::new();

        input.lines().for_each(|line| {
            let parts: Vec<u128> = line
                .split(" ")
                .filter(|s| !s.is_empty())
                .flat_map(|s| (*s).parse::<u128>())
                .collect();

            left_list.push(parts[0]);
            right_counts
                .entry(parts[1])
                .and_modify(|c| *c += 1)
                .or_insert(1);
        });

        left_list
            .iter()
            .fold(0u128, |acc, x| match right_counts.get(&x) {
                None => acc,
                Some(i) => acc + (*x * *i)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const P: Problem1 = Problem1 {};

    #[test]
    fn should_sort_and_sum_differences_when_0() {
        let sample_input = "\
        3 1\n\
        2 2\n\
        1 3";

        assert_eq!(P.part1(sample_input), 0);
    }

    #[test]
    fn should_find_when_numbers_are_different() {
        let sample_input = "\
        4 1\n\
        2 2\n\
        1 3";
        assert_eq!(P.part1(sample_input), 1);
    }

    #[test]
    fn test_should_calculate_similarity_score_for_part2_using_sample_data() {
        let sample_input = "\
        3   4\n\
        4   3\n\
        2   5\n\
        1   3\n\
        3   9\n\
        3   3";
        assert_eq!(P.part2(sample_input), 31);
    }

    #[test]
    fn should_calculate_similarity_score_for_very_similar_list() {
        let sample_input = "7  7";
        assert_eq!(P.part2(sample_input), 7);
    }

    #[test]
    fn should_ignore_elements_not_found_in_right_list() {
        let sample_input = "\
        7  7\n\
        2  6";
        assert_eq!(P.part2(sample_input), 7);
    }

    #[test]
    fn should_reflect_number_of_times_element_appears_in_right() {
        let sample_input = "\
        7  7\n\
        2  7";
        assert_eq!(P.part2(sample_input), 14);
    }

    #[test]
    fn should_calculate_similarity_score_for_very_similar_list_with_repeats() {
        let sample_input = "\
        7  7\n\
        7  7";
        assert_eq!(P.part2(sample_input), 7 * 2 * 2);
    }
}

pub const PROBLEM1: Problem1 = Problem1 {};
