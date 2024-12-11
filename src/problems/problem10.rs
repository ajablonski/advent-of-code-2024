use crate::problems::common::Grid;
use crate::problems::Problem;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub struct Problem10 {}

impl Problem<u128> for Problem10 {
    fn part1(&self, input: &str) -> u128 {
        let grid: Grid<u32> = Grid::from_string(input);

        grid.clone()
            .into_iter()
            .filter(|(_, val)| *val == 0)
            .map(|th| {
                let mut to_explore = VecDeque::from([th.clone()]);
                let mut trail_ends: HashSet<((i32, i32), u32)> = HashSet::new();

                while !to_explore.is_empty() {
                    let ((coords_row, coords_col), val) = to_explore.pop_front().unwrap();
                    let next_steps = vec![
                        (coords_row - 1, coords_col),
                        (coords_row, coords_col + 1),
                        (coords_row + 1, coords_col),
                        (coords_row, coords_col - 1),
                    ]
                    .iter()
                    .filter(|(row_num, col_num)| {
                        if (*row_num) >= 0 && (*col_num) >= 0 {
                            grid.lines.get(*row_num as usize).is_some_and(|row| {
                                row.get(*col_num as usize).is_some_and(|v| *v == val + 1)
                            })
                        } else {
                            false
                        }
                    })
                    .map(|coords| (coords.clone(), val + 1))
                    .collect_vec();

                    if val == 8 {
                        trail_ends.extend(next_steps);
                    } else {
                        to_explore.extend(next_steps);
                    }
                }

                trail_ends.len() as u128
            })
            .sum()
    }

    fn part2(&self, input: &str) -> u128 {
        let grid: Grid<u32> = Grid::from_string(input);

        grid.clone()
            .into_iter()
            .filter(|(_, val)| *val == 0)
            .map(|th| {
                let mut to_explore = VecDeque::from([vec![th.clone()]]);
                let mut paths: HashSet<Vec<((i32, i32), u32)>> = HashSet::new();

                while !to_explore.is_empty() {
                    let path_so_far = to_explore.pop_front().unwrap();
                    let ((coords_row, coords_col), val) = path_so_far.last().unwrap();
                    let new_paths_so_far = vec![
                        (*coords_row - 1, *coords_col),
                        (*coords_row, *coords_col + 1),
                        (*coords_row + 1, *coords_col),
                        (*coords_row, *coords_col - 1),
                    ]
                    .iter()
                    .filter(|(row_num, col_num)| {
                        if (*row_num) >= 0 && (*col_num) >= 0 {
                            grid.lines.get(*row_num as usize).is_some_and(|row| {
                                row.get(*col_num as usize).is_some_and(|v| *v == *val + 1)
                            })
                        } else {
                            false
                        }
                    })
                    .map(|coords| {
                        let mut new_vec = path_so_far.clone();

                        new_vec.push((coords.clone(), val + 1));

                        new_vec
                    })
                    .collect_vec();

                    if *val == 8 {
                        paths.extend(new_paths_so_far);
                    } else {
                        to_explore.extend(new_paths_so_far);
                    }
                }

                paths.len() as u128
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_produce_correct_answer_for_part_1() {
        let p = Problem10 {};
        assert_eq!(
            p.part1(
                "\
                89010123\n\
                78121874\n\
                87430965\n\
                96549874\n\
                45678903\n\
                32019012\n\
                01329801\n\
                10456732
                "
            ),
            36
        );
    }

    #[test]
    fn should_produce_correct_answer_for_part_2() {
        let p = Problem10 {};
        assert_eq!(
            p.part2(
                "\
                89010123\n\
                78121874\n\
                87430965\n\
                96549874\n\
                45678903\n\
                32019012\n\
                01329801\n\
                10456732"
            ),
            81
        )
    }
}
