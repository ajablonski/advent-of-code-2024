use crate::problems::Problem;
use crate::Event;
use std::ops::Add;
use std::sync::mpsc::Sender;

pub struct Problem4 {}

impl Problem<u128> for Problem4 {
    fn part1(&self, input: &str, tx: Sender<Event>) -> u128 {
        let horizontal_count = input
            .lines()
            .map(|l| Self::find_xmas(&l.to_string()))
            .sum::<usize>();

        let vertical_count = Self::get_vertical_lines(input, &tx)
            .iter()
            .map(Self::find_xmas)
            .sum::<usize>();

        let diagonal_count = Self::get_diagonal_lines(input, &tx)
            .iter()
            .map(Self::find_xmas)
            .sum::<usize>();

        horizontal_count as u128 + vertical_count as u128 + diagonal_count as u128
    }

    fn part2(&self, input: &str, _tx: Sender<Event>) -> u128 {
        let lines: Vec<Vec<char>> = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();

        let row_count = lines.len();

        let column_count = lines[0].len();

        (0..row_count - 2)
            .map(|i| {
                (0..column_count - 2)
                    .filter(|j| {
                        (i < row_count - 2)
                            && (*j < column_count - 2)
                            && (lines[i + 1][*j + 1] == 'A')
                            && ((lines[i][*j] == 'M'
                                && lines[i + 2][*j + 2] == 'S'
                                && lines[i + 2][*j] == 'M'
                                && lines[i][*j + 2] == 'S')
                                || (lines[i][*j] == 'M'
                                    && lines[i + 2][*j + 2] == 'S'
                                    && lines[i + 2][*j] == 'S'
                                    && lines[i][*j + 2] == 'M')
                                || (lines[i][*j] == 'S'
                                    && lines[i + 2][*j + 2] == 'M'
                                    && lines[i + 2][*j] == 'M'
                                    && lines[i][*j + 2] == 'S')
                                || (lines[i][*j] == 'S'
                                    && lines[i + 2][*j + 2] == 'M'
                                    && lines[i + 2][*j] == 'S'
                                    && lines[i][*j + 2] == 'M'))
                    })
                    .count() as u128
            })
            .sum()
    }
}

impl Problem4 {
    fn find_xmas(line: &String) -> usize {
        line.matches("XMAS").count() + line.matches("SAMX").count()
    }

    fn get_vertical_lines(input: &str, _tx: &Sender<Event>) -> Vec<String> {
        let column_count = input.lines().next().unwrap().len();

        let mut strs: Vec<String> = Vec::new();

        for i in 0..column_count {
            strs.push(
                input
                    .lines()
                    .fold(String::new(), |result, l| result.add(&l[i..i + 1])),
            );
        }

        strs
    }

    fn get_diagonal_lines(input: &str, _tx: &Sender<Event>) -> Vec<String> {
        let column_count = input.lines().next().unwrap().len();

        let row_count = input.lines().count();

        let mut negative_slope_strs: Vec<String> =
            vec![String::new(); column_count + row_count - 1];

        input.lines().enumerate().for_each(|(i, line)| {
            line.char_indices().for_each(|(j, c)| {
                let i1 = row_count + j - i - 1;
                negative_slope_strs[i1].push(c)
            });
        });

        let mut positive_slope_strs: Vec<String> =
            vec![String::new(); column_count + row_count - 1];

        input.lines().enumerate().for_each(|(i, line)| {
            line.char_indices().for_each(|(j, c)| {
                let i1 = i + j;
                positive_slope_strs[i1].push(c)
            });
        });

        negative_slope_strs.append(positive_slope_strs.as_mut());

        negative_slope_strs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    const P: Problem4 = Problem4 {};

    #[test]
    fn should_solve_part_1_example() {
        assert_eq!(
            P.part1(
                "\
                MMMSXXMASM\n\
                MSAMXMSMSA\n\
                AMXSXMAAMM\n\
                MSAMASMSMX\n\
                XMASAMXAMM\n\
                XXAMMXXAMA\n\
                SMSMSASXSS\n\
                SAXAMASAAA\n\
                MAMMMXMMMM\n\
                MXMXAXMASX\
                ",
                mpsc::channel().0
            ),
            18
        );
    }

    #[test]
    fn should_find_forward_horizontal() {
        assert_eq!(
            P.part1(
                "\
                MMMXMASXXX\
                ",
                mpsc::channel().0
            ),
            1
        );
    }

    #[test]
    fn should_find_backward_horizontal() {
        assert_eq!(
            P.part1(
                "\
                MMMMMSAMXXXXXX\
                ",
                mpsc::channel().0
            ),
            1
        );
    }

    #[test]
    fn should_find_forward_vertical() {
        assert_eq!(
            P.part1(
                "\
                MMMMMXMMMMM\n\
                MMMMMMMMMMM\n\
                MMMMMAMMMMM\n\
                MMMMMSMMMMM\
                ",
                mpsc::channel().0
            ),
            1
        );
    }

    #[test]
    fn should_find_backward_vertical() {
        assert_eq!(
            P.part1(
                "\
                MMMMMSMMMMM\n\
                MMMMMAMMMMM\n\
                MMMMMMMMMMM\n\
                MMMMMXMMMMM\
                ",
                mpsc::channel().0
            ),
            1
        );
    }

    #[test]
    fn should_find_forward_negative_diagonal() {
        assert_eq!(
            P.part1(
                "\
                MMMXMMMMMMM\n\
                MMMMMMMMMMM\n\
                MMMMMAMMMMM\n\
                MMMMMMSMMMM\
                ",
                mpsc::channel().0
            ),
            1
        );
    }

    #[test]
    fn should_find_backward_negative_diagonal() {
        assert_eq!(
            P.part1(
                "\
                MMMSMMMMMMM\n\
                MMMMAMMMMMM\n\
                MMMMMMMMMMM\n\
                MMMMMMXMMMM\
                ",
                mpsc::channel().0
            ),
            1
        );
    }

    #[test]
    fn should_find_forward_positive_diagonal() {
        assert_eq!(
            P.part1(
                "\
                MMMMMSMMMMM\n\
                MMMMAMMMMMM\n\
                MMMMMMMMMMM\n\
                MMXMMMMMMMM\
                ",
                mpsc::channel().0
            ),
            1
        );
    }

    #[test]
    fn should_find_backward_positive_diagonal() {
        assert_eq!(
            P.part1(
                "\
                MMMMMXMMMMM\n\
                MMMMMMMMMMM\n\
                MMMAMMMMMMM\n\
                MMSMMMMMMMM\
                ",
                mpsc::channel().0
            ),
            1
        );
    }

    #[test]
    fn should_solve_part_2_example() {
        assert_eq!(
            P.part2(
                "\
                MMMSXXMASM\n\
                MSAMXMSMSA\n\
                AMXSXMAAMM\n\
                MSAMASMSMX\n\
                XMASAMXAMM\n\
                XXAMMXXAMA\n\
                SMSMSASXSS\n\
                SAXAMASAAA\n\
                MAMMMXMMMM\n\
                MXMXAXMASX\
                ",
                mpsc::channel().0
            ),
            9
        );
    }
}
