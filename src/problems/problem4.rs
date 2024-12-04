use crate::problems::Problem;
use crate::Event;
use std::fmt;
use std::fmt::Write;
use std::sync::mpsc::Sender;

pub struct Problem4 {}

struct Grid {
    lines: Vec<Vec<char>>,
    row_count: usize,
    col_count: usize,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('\n')?;
        self.lines.iter().for_each(|line| {
            line.iter().for_each(|c| f.write_char(*c).unwrap());

            f.write_char('\n').unwrap();
        });

        Ok(())
    }
}

impl Grid {
    fn from_lines(lines: Vec<Vec<char>>) -> Self {
        Self {
            row_count: lines.len(),
            col_count: lines[0].len(),
            lines,
        }
    }

    fn from_string(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();

        Self::from_lines(lines)
    }
}

impl Problem<u128> for Problem4 {
    fn part1(&self, input: &str, _tx: Sender<Event>) -> u128 {
        let grid = Grid::from_string(input);

        (0..grid.row_count)
            .map(|i| {
                (0..grid.col_count)
                    .map(|j| {
                        Self::check_horizontal(i, j, &grid)
                            + Self::check_vertical(i, j, &grid)
                            + Self::check_diagonal(i, j, &grid)
                    })
                    .sum::<u128>() as u128
            })
            .sum()
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
    fn check_horizontal(row: usize, column: usize, grid: &Grid) -> u128 {
        let mut total = 0;
        let word_as_chars = "XMAS".chars().collect::<Vec<char>>();
        let word_length = word_as_chars.len();
        if grid.lines[row][column] == word_as_chars[0] {
            if column >= word_length - 1
                && grid.lines[row][column - 1] == word_as_chars[1]
                && grid.lines[row][column - 2] == word_as_chars[2]
                && grid.lines[row][column - 3] == word_as_chars[3]
            {
                total += 1
            }
            if column + word_length - 1 < grid.col_count
                && grid.lines[row][column + 1] == word_as_chars[1]
                && grid.lines[row][column + 2] == word_as_chars[2]
                && grid.lines[row][column + 3] == word_as_chars[3]
            {
                total += 1
            }
        }
        total
    }

    fn check_diagonal(row: usize, column: usize, grid: &Grid) -> u128 {
        let word_as_chars = "XMAS".chars().collect::<Vec<char>>();
        let word_length = word_as_chars.len();
        let mut total = 0;
        if grid.lines[row][column] == word_as_chars[0]{
            if row >= word_length - 1 && column + word_length - 1 < grid.col_count // /, upwards
                && grid.lines[row - 1][column + 1] == word_as_chars[1]
                && grid.lines[row - 2][column + 2] == word_as_chars[2]
                && grid.lines[row - 3][column + 3] == word_as_chars[3]
            {
                total += 1
            }
            if row + word_length - 1 < grid.row_count && column + word_length - 1 < grid.col_count // \, downwards
                && grid.lines[row + 1][column + 1] == word_as_chars[1]
                && grid.lines[row + 2][column + 2] == word_as_chars[2]
                && grid.lines[row + 3][column + 3] == word_as_chars[3]
            {
                total += 1
            }
            if row + word_length - 1 < grid.row_count && column >= word_length - 1 // /, downwards
                && grid.lines[row + 1][column - 1] == word_as_chars[1]
                && grid.lines[row + 2][column - 2] == word_as_chars[2]
                && grid.lines[row + 3][column - 3] == word_as_chars[3]
            {
                total += 1
            }
            if row >= word_length - 1 && column >= word_length - 1 // \, upwards
                && grid.lines[row - 1][column - 1] == word_as_chars[1]
                && grid.lines[row - 2][column - 2] == word_as_chars[2]
                && grid.lines[row - 3][column - 3] == word_as_chars[3]
            {
                total += 1
            }
        }

        total
    }

    fn check_vertical(row: usize, column: usize, grid: &Grid) -> u128 {
        let word_as_chars = "XMAS".chars().collect::<Vec<char>>();
        let word_length = word_as_chars.len();
        let mut total = 0;
        if grid.lines[row][column] == word_as_chars[0] {
            if row >= word_length - 1
                && grid.lines[row - 1][column] == word_as_chars[1]
                && grid.lines[row - 2][column] == word_as_chars[2]
                && grid.lines[row - 3][column] == word_as_chars[3] {
                total += 1
            }

            if row + 3 < grid.row_count
                && grid.lines[row + 1][column] == word_as_chars[1]
                && grid.lines[row + 2][column] == word_as_chars[2]
                && grid.lines[row + 3][column] == word_as_chars[3] {
                total += 1
            }
        }
        total
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
