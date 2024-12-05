use crate::problems::problem4::DiagonalTypes::{DownLeft, DownRight, UpLeft, UpRight};
use crate::problems::Problem;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Write;

pub struct Problem4 {}

struct Grid {
    lines: Vec<Vec<char>>,
    row_count: usize,
    col_count: usize,
}

#[derive(Eq, Hash, PartialEq)]
enum DiagonalTypes {
    UpRight,
    DownRight,
    DownLeft,
    UpLeft,
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
    fn part1(&self, _input: &str) -> u128 {
        let grid = Grid::from_string(_input);
        let word = "XMAS";

        (0..grid.row_count)
            .map(|i| {
                (0..grid.col_count)
                    .map(|j| {
                        Self::check_horizontal(i, j, &grid, word)
                            + Self::check_vertical(i, j, &grid, word)
                            + Self::check_diagonal(i, j, &grid, word).len() as u128
                    })
                    .sum::<u128>() as u128
            })
            .sum()
    }

    fn part2(&self, _input: &str) -> u128 {
        let grid = Grid::from_string(_input);

        (0..grid.row_count)
            .map(|i| {
                (0..grid.col_count)
                    .filter(|j| {
                        let as_diagonals = Self::check_diagonal(i, *j, &grid, "AS");

                        let am_diagonals = Self::check_diagonal(i, *j, &grid, "AM");

                        as_diagonals.len() == 2
                            && am_diagonals.len() == 2
                            && (as_diagonals != HashSet::from([UpLeft, DownRight]))
                            && (as_diagonals != HashSet::from([UpRight, DownLeft]))
                    })
                    .count() as u128
            })
            .sum::<u128>()
    }
}

impl Problem4 {
    fn check_horizontal(row: usize, column: usize, grid: &Grid, word: &str) -> u128 {
        let mut total = 0;
        let word_as_chars = word.chars().collect::<Vec<char>>();
        let word_length = word_as_chars.len();
        if column >= word_length - 1
            && word_as_chars
                .iter()
                .enumerate()
                .all(|(i, c)| grid.lines[row][column - i] == *c)
        {
            // check backwards
            total += 1
        }
        if column + word_length - 1 < grid.col_count
            && word_as_chars
                .iter()
                .enumerate()
                .all(|(i, c)| grid.lines[row][column + i] == *c)
        {
            // check forwards
            total += 1
        }
        total
    }

    fn check_diagonal(
        row: usize,
        column: usize,
        grid: &Grid,
        word: &str,
    ) -> HashSet<DiagonalTypes> {
        let word_as_chars = word.chars().collect::<Vec<char>>();
        let word_length = word_as_chars.len();
        let mut match_types: HashSet<DiagonalTypes> = HashSet::new();
        if row >= word_length - 1 && column + word_length - 1 < grid.col_count // /, upwards
            && word_as_chars
            .iter()
            .enumerate()
            .all(|(i, c)| grid.lines[row - i][column + i] == *c)
        {
            match_types.insert(DiagonalTypes::UpRight);
        }
        if row + word_length - 1 < grid.row_count && column + word_length - 1 < grid.col_count // \, downwards
            && word_as_chars
            .iter()
            .enumerate()
            .all(|(i, c)| grid.lines[row + i][column + i] == *c)
        {
            match_types.insert(DiagonalTypes::DownRight);
        }
        if row + word_length - 1 < grid.row_count && column >= word_length - 1 // /, downwards
            && word_as_chars
            .iter()
            .enumerate()
            .all(|(i, c)| grid.lines[row + i][column - i] == *c)
        {
            match_types.insert(DiagonalTypes::DownLeft);
        }
        if row >= word_length - 1 && column >= word_length - 1 // \, upwards
            && word_as_chars
            .iter()
            .enumerate()
            .all(|(i, c)| grid.lines[row - i][column - i] == *c)
        {
            match_types.insert(DiagonalTypes::UpLeft);
        }

        match_types
    }

    fn check_vertical(row: usize, column: usize, grid: &Grid, word: &str) -> u128 {
        let word_as_chars = word.chars().collect::<Vec<char>>();
        let word_length = word_as_chars.len();
        let mut total = 0;
        if row >= word_length - 1
            && word_as_chars
                .iter()
                .enumerate()
                .all(|(i, c)| grid.lines[row - i][column] == *c)
        {
            total += 1
        }

        if row + 3 < grid.row_count
            && word_as_chars
                .iter()
                .enumerate()
                .all(|(i, c)| grid.lines[row + i][column] == *c)
        {
            total += 1
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const P: Problem4 = Problem4 {};

    #[test]
    fn should_solve_part_1_example() {
        assert_eq!(
            P.part1("\
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
            "),
            18
        );
    }

    #[test]
    fn should_find_forward_horizontal() {
        assert_eq!(
            P.part1("\
            MMMXMASXXX\
            "),
            1
        );
    }

    #[test]
    fn should_find_backward_horizontal() {
        assert_eq!(
            P.part1("\
            MMMMMSAMXXXXXX\
            "),
            1
        );
    }

    #[test]
    fn should_find_forward_vertical() {
        assert_eq!(
            P.part1("\
            MMMMMXMMMMM\n\
            MMMMMMMMMMM\n\
            MMMMMAMMMMM\n\
            MMMMMSMMMMM\
            "),
            1
        );
    }

    #[test]
    fn should_find_backward_vertical() {
        assert_eq!(
            P.part1("\
            MMMMMSMMMMM\n\
            MMMMMAMMMMM\n\
            MMMMMMMMMMM\n\
            MMMMMXMMMMM\
            "),
            1
        );
    }

    #[test]
    fn should_find_forward_negative_diagonal() {
        assert_eq!(
            P.part1("\
            MMMXMMMMMMM\n\
            MMMMMMMMMMM\n\
            MMMMMAMMMMM\n\
            MMMMMMSMMMM\
            "),
            1
        );
    }

    #[test]
    fn should_find_backward_negative_diagonal() {
        assert_eq!(
            P.part1("\
            MMMSMMMMMMM\n\
            MMMMAMMMMMM\n\
            MMMMMMMMMMM\n\
            MMMMMMXMMMM\
            "),
            1
        );
    }

    #[test]
    fn should_find_forward_positive_diagonal() {
        assert_eq!(
            P.part1("\
            MMMMMSMMMMM\n\
            MMMMAMMMMMM\n\
            MMMMMMMMMMM\n\
            MMXMMMMMMMM\
            "),
            1
        );
    }

    #[test]
    fn should_find_backward_positive_diagonal() {
        assert_eq!(
            P.part1("\
            MMMMMXMMMMM\n\
            MMMMMMMMMMM\n\
            MMMAMMMMMMM\n\
            MMSMMMMMMMM\
            "),
            1
        );
    }

    #[test]
    fn should_solve_part_2_example() {
        assert_eq!(
            P.part2("\
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
            "),
            9
        );
    }

    #[test]
    fn should_correctly_identify_x_mas() {
        assert_eq!(
            P.part2("\
            MAS\n\
            AAA\n\
            SAM"),
            0
        );

        assert_eq!(
            P.part2("\
            MAS\n\
            AAA\n\
            MAS"),
            1
        )
    }
}
