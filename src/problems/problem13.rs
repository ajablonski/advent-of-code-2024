use crate::problems::Problem;
use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};
use regex::Regex;

pub struct Problem13 {}

#[derive(Debug, PartialEq, Eq)]
struct Machine {
    a_button: (u64, u64),
    b_button: (u64, u64),
    prize_location: (u64, u64),
}

impl Problem<u128> for Problem13 {
    fn part1(&self, input: &str) -> u128 {
        Problem13::solve(input, 0)
    }

    fn part2(&self, input: &str) -> u128 {
        Problem13::solve(input, 10000000000000)
    }
}

impl Problem13 {
    const FLOAT_THRESHOLD: f64 = 0.001;

    fn solve(input: &str, offset: u64) -> u128 {
        Problem13::parse(input, offset)
            .iter()
            .map(|machine| {
                let final_vector_in_x_y = Vector2::new(
                    machine.prize_location.0 as f64,
                    machine.prize_location.1 as f64,
                );
                let change_of_basis_matrix = Matrix2::new(
                    machine.a_button.0 as f64,
                    machine.b_button.0 as f64,
                    machine.a_button.1 as f64,
                    machine.b_button.1 as f64,
                );
                let inverse = change_of_basis_matrix.try_inverse().unwrap();

                let vector_in_new_basis = inverse * final_vector_in_x_y;
                if (vector_in_new_basis.x - vector_in_new_basis.x.round()).abs()
                    < Self::FLOAT_THRESHOLD
                    && (vector_in_new_basis.y - vector_in_new_basis.y.round()).abs()
                        < Self::FLOAT_THRESHOLD
                {
                    vector_in_new_basis.x.round() as u128 * 3
                        + vector_in_new_basis.y.round() as u128
                } else {
                    0
                }
            })
            .sum()
    }

    fn parse(input: &str, offset: u64) -> Vec<Machine> {
        let machine_regex: Regex = Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
        .unwrap();

        input
            .split("\n\n")
            .map(
                |item| match machine_regex.captures(item.trim()).unwrap().extract() {
                    (_, [a_x, a_y, b_x, b_y, p_x, p_y]) => Machine {
                        a_button: (a_x.parse::<u64>().unwrap(), a_y.parse::<u64>().unwrap()),
                        b_button: (b_x.parse::<u64>().unwrap(), b_y.parse::<u64>().unwrap()),
                        prize_location: (
                            p_x.parse::<u64>().unwrap() + offset,
                            p_y.parse::<u64>().unwrap() + offset,
                        ),
                    },
                },
            )
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_produce_correct_answer_for_part_1() {
        let p = Problem13 {};
        assert_eq!(
            p.part1(
                "\
            Button A: X+94, Y+34\n\
            Button B: X+22, Y+67\n\
            Prize: X=8400, Y=5400\n\
            \n\
            Button A: X+26, Y+66\n\
            Button B: X+67, Y+21\n\
            Prize: X=12748, Y=12176\n\
            \n\
            Button A: X+17, Y+86\n\
            Button B: X+84, Y+37\n\
            Prize: X=7870, Y=6450\n\
            \n\
            Button A: X+69, Y+23\n\
            Button B: X+27, Y+71\n\
            Prize: X=18641, Y=10279\n\
            "
            ),
            480
        );
    }

    #[test]
    fn should_produce_correct_answer_for_part_2() {
        let p = Problem13 {};
        assert_eq!(
            p.part2(
                "\
            Button A: X+94, Y+34\n\
            Button B: X+22, Y+67\n\
            Prize: X=8400, Y=5400\n\
            \n\
            Button A: X+26, Y+66\n\
            Button B: X+67, Y+21\n\
            Prize: X=12748, Y=12176\n\
            \n\
            Button A: X+17, Y+86\n\
            Button B: X+84, Y+37\n\
            Prize: X=7870, Y=6450\n\
            \n\
            Button A: X+69, Y+23\n\
            Button B: X+27, Y+71\n\
            Prize: X=18641, Y=10279\n\
            "
            ),
            875318608908
        );
    }

    mod test_helpers {
        use super::*;

        mod test_parse {
            use super::*;

            #[test]
            fn should_parse_input_correctly() {
                assert_eq!(
                    Problem13::parse(
                        "\
                        Button A: X+1, Y+2\n\
                        Button B: X+3, Y+4\n\
                        Prize: X=5, Y=6\n\
                        \n\
                        Button A: X+7, Y+8\n\
                        Button B: X+9, Y+10\n\
                        Prize: X=11, Y=12\n",
                        10
                    ),
                    vec![
                        Machine {
                            a_button: (1, 2),
                            b_button: (3, 4),
                            prize_location: (15, 16)
                        },
                        Machine {
                            a_button: (7, 8),
                            b_button: (9, 10),
                            prize_location: (21, 22)
                        }
                    ]
                )
            }
        }
    }
}
