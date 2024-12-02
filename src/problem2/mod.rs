use crate::Problem;

pub struct Problem2 {}

#[derive(Debug, PartialEq, Eq)]
enum ReportType {
    Unknown,
    Increasing,
    Decreasing,
    Unsafe,
}

impl Problem<u128> for Problem2 {
    fn part1(&self, input: &str) -> u128 {
        input
            .lines()
            .filter(|l| {
                self.is_safe(
                    &l.split(" ").map(|n| n.parse::<i8>().unwrap()).collect::<Vec<i8>>()[..],
                    false,
                )
            })
            .count() as u128
    }

    fn part2(&self, input: &str) -> u128 {
        input
            .lines()
            .filter(|l| {
                let ints = &*l.split(" ").map(|n| n.parse::<i8>().unwrap()).collect::<Vec<i8>>();

                self.is_safe(ints, true)
            })
            .count() as u128
    }
}
impl Problem2 {
    pub(crate) fn is_safe(&self, report: &[i8], tolerate_bad_level: bool) -> bool {
        if tolerate_bad_level {
            let mut slice_options: Vec<Vec<i8>> = Vec::new();

            slice_options.push(report.to_vec());

            for i in 0..report.len() {
                let other_possibility = [&report[0..i], &report[i + 1..]].concat();
                slice_options.push(other_possibility)
            }

            slice_options.iter().any(|option| self.is_safe(option, false))
        } else {
            report
                .iter()
                .fold((ReportType::Unknown, None), |(r, last), w| match last {
                    Some(n) => {
                        let difference = w - n;

                        match r {
                            ReportType::Unknown => {
                                if difference > 0 && difference <= 3 {
                                    (ReportType::Increasing, Some(w))
                                } else if difference < 0 && difference >= -3 {
                                    (ReportType::Decreasing, Some(w))
                                } else {
                                    (ReportType::Unsafe, Some(w))
                                }
                            }
                            ReportType::Increasing => {
                                if difference > 0 && difference <= 3 {
                                    (ReportType::Increasing, Some(w))
                                } else {
                                    (ReportType::Unsafe, Some(w))
                                }
                            }
                            ReportType::Decreasing => {
                                if difference < 0 && difference >= -3 {
                                    (ReportType::Decreasing, Some(w))
                                } else {
                                    (ReportType::Unsafe, Some(w))
                                }
                            }
                            ReportType::Unsafe => (ReportType::Unsafe, Some(w)),
                        }
                    }
                    None => (ReportType::Unknown, Some(w)),
                }).0 != ReportType::Unsafe
        }
    }
}

pub const PROBLEM2: Problem2 = Problem2 {};

#[cfg(test)]
mod tests {
    use super::*;
    const P: Problem2 = Problem2 {};

    #[test]
    fn should_return_correct_response_for_part1_example() {
        let result = P.part1(
            "\
        7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9",
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn is_safe_should_allow_increasing_small_interval_report() {
        assert!(P.is_safe(&[1, 2, 5, 7, 8], false))
    }

    #[test]
    fn is_safe_should_allow_increasing_small_interval_report_with_1_error_when_tolerated() {
        assert!(P.is_safe(&[9, 2, 5, 7, 8], true));
        assert!(P.is_safe(&[1, 9, 4, 7, 8], true));
        assert!(P.is_safe(&[1, 2, 9, 5, 8], true));
        assert!(P.is_safe(&[1, 2, 5, 9, 8], true));
        assert!(P.is_safe(&[1, 2, 5, 7, 15], true));
        assert!(P.is_safe(&[4, 2, 5, 7, 9], true))
    }

    #[test]
    fn is_safe_should_allow_decreasing_small_interval_report() {
        assert!(P.is_safe(&[8, 7, 6, 3, 1], false))
    }

    #[test]
    fn is_safe_should_allow_decreasing_small_interval_report_with_1_error_when_tolerated() {
        assert!(P.is_safe(&[15, 7, 6, 3, 1], true));
        assert!(P.is_safe(&[9, 15, 6, 3, 1], true));
        assert!(P.is_safe(&[9, 7, 15, 4, 2], true));
        assert!(P.is_safe(&[9, 7, 6, 15, 4], true));
        assert!(P.is_safe(&[9, 7, 6, 3, 15], true));
        assert!(P.is_safe(&[6, 7, 6, 3, 1], true))
    }

    #[test]
    fn is_safe_should_disallow_decreasing_too_big_interval() {
        assert!(!P.is_safe(&[8, 7, 6, 2, 1], false))
    }

    #[test]
    fn is_safe_should_disallow_increasing_too_big_interval() {
        assert!(!P.is_safe(&[1, 2, 6, 7, 8], false))
    }

    #[test]
    fn is_safe_should_disallow_non_monotonic_intervals() {
        assert!(!P.is_safe(&[1, 2, 0, 4, 5], false))
    }

    #[test]
    fn should_return_correct_response_for_part2_example() {
        let result = P.part2(
            "\
        7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9",
        );
        assert_eq!(result, 4);
    }
}
