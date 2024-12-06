use crate::display::AppDisplayState;
use crate::problems::Problem;
use crate::Event;
use crate::Event::UpdateAppDisplayState;
use std::sync::mpsc::Sender;

pub struct Problem2 {
    tx: Sender<Event>,
}

#[derive(Debug, PartialEq, Eq)]
enum ReportType {
    Unknown,
    Increasing,
    Decreasing,
    Unsafe,
}

impl Problem<u128> for Problem2 {
    fn part1(&self, _input: &str) -> u128 {
        let mut partial_count = 0;
        self.parse(_input)
            .iter()
            .filter(|l| {
                let is_safe = self.is_safe(l);

                if is_safe {
                    partial_count += 1;
                }

                self.tx
                    .send(UpdateAppDisplayState(AppDisplayState::part_1_only(partial_count)))
                    .unwrap_or_else(|e| println!("Error updating UI {:?}", e));

                is_safe
            })
            .count() as u128
    }

    fn part2(&self, _input: &str) -> u128 {
        self.parse(_input)
            .iter()
            .filter(|l| self.is_safe_with_bad_level(l))
            .count() as u128
    }
}

impl Problem2 {
    fn is_safe_with_bad_level(&self, report: &[i8]) -> bool {
        let mut slice_options: Vec<Vec<i8>> = Vec::new();

        slice_options.push(report.to_vec());

        for i in 0..report.len() {
            let other_possibility = [&report[0..i], &report[i + 1..]].concat();
            slice_options.push(other_possibility)
        }

        slice_options.iter().any(|option| self.is_safe(option))
    }

    fn is_safe(&self, report: &[i8]) -> bool {
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
            })
            .0
            != ReportType::Unsafe
    }

    fn parse(&self, input: &str) -> Vec<Vec<i8>> {
        input
            .lines()
            .map(|l| l.split(" ").map(|n| n.parse::<i8>().unwrap()).collect())
            .collect()
    }

    pub(crate) fn new(tx: &Sender<Event>) -> Problem2 {
        Problem2 { tx: tx.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn should_return_correct_response_for_part1_example() {
        let p: Problem2 = Problem2::new(&mpsc::channel().0);

        let result = p.part1(
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
        let p: Problem2 = Problem2::new(&mpsc::channel().0);

        assert!(p.is_safe(&[1, 2, 5, 7, 8]))
    }

    #[test]
    fn is_safe_should_allow_increasing_small_interval_report_with_1_error_when_tolerated() {
        let p: Problem2 = Problem2::new(&mpsc::channel().0);

        assert!(p.is_safe_with_bad_level(&[9, 2, 5, 7, 8]));
        assert!(p.is_safe_with_bad_level(&[1, 9, 4, 7, 8]));
        assert!(p.is_safe_with_bad_level(&[1, 2, 9, 5, 8]));
        assert!(p.is_safe_with_bad_level(&[1, 2, 5, 9, 8]));
        assert!(p.is_safe_with_bad_level(&[1, 2, 5, 7, 15]));
        assert!(p.is_safe_with_bad_level(&[4, 2, 5, 7, 9]))
    }

    #[test]
    fn is_safe_should_allow_decreasing_small_interval_report() {
        let p: Problem2 = Problem2::new(&mpsc::channel().0);

        assert!(p.is_safe(&[8, 7, 6, 3, 1]))
    }

    #[test]
    fn is_safe_should_allow_decreasing_small_interval_report_with_1_error_when_tolerated() {
        let p: Problem2 = Problem2::new(&mpsc::channel().0);

        assert!(p.is_safe_with_bad_level(&[15, 7, 6, 3, 1]));
        assert!(p.is_safe_with_bad_level(&[9, 15, 6, 3, 1]));
        assert!(p.is_safe_with_bad_level(&[9, 7, 15, 4, 2]));
        assert!(p.is_safe_with_bad_level(&[9, 7, 6, 15, 4]));
        assert!(p.is_safe_with_bad_level(&[9, 7, 6, 3, 15]));
        assert!(p.is_safe_with_bad_level(&[6, 7, 6, 3, 1]))
    }

    #[test]
    fn is_safe_should_disallow_decreasing_too_big_interval() {
        let p: Problem2 = Problem2::new(&mpsc::channel().0);

        assert!(!p.is_safe(&[8, 7, 6, 2, 1]))
    }

    #[test]
    fn is_safe_should_disallow_increasing_too_big_interval() {
        let p: Problem2 = Problem2::new(&mpsc::channel().0);

        assert!(!p.is_safe(&[1, 2, 6, 7, 8]))
    }

    #[test]
    fn is_safe_should_disallow_non_monotonic_intervals() {
        let p: Problem2 = Problem2::new(&mpsc::channel().0);

        assert!(!p.is_safe(&[1, 2, 0, 4, 5]))
    }

    #[test]
    fn should_return_correct_response_for_part2_example() {
        let p: Problem2 = Problem2::new(&mpsc::channel().0);

        let result = p.part2(
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
