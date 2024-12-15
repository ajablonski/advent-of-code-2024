use crate::display::AppDisplayState;
use crate::problems::common::Grid;
use crate::problems::Problem;
use crate::Event;
use crate::Event::UpdateAppDisplayState;
use itertools::Itertools;
use regex::Regex;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

pub struct Problem14 {
    pub(crate) tx: mpsc::Sender<Event>,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn mv(&mut self, steps: isize, grid_width: isize, grid_height: isize) {
        self.position = (
            (self.position.0 + (self.velocity.0 * steps)).rem_euclid(grid_width),
            (self.position.1 + (self.velocity.1 * steps)).rem_euclid(grid_height),
        )
    }
}

impl Problem<u128> for Problem14 {
    fn part1(&self, input: &str) -> u128 {
        Problem14::solve(input, 101, 103, 100)
    }

    fn part2(&self, input: &str) -> u128 {
        let robots = Problem14::parse(input);
        let grid_width = 101;
        let grid_height = 103;

        let most_robots_in_row = |robots: &Vec<Robot>, _grid_width: isize, grid_height: isize| {
            (0..grid_height)
                .map(|row_num| robots
                    .iter()
                    .filter(|&r| r.position.1 == row_num)
                    .map(|&r| r.position.0)
                    .sorted()
                    .dedup()
                    .count())
                .max()
                .unwrap() as u128
        };

        let answer = (0..10000)
            .rev()
            .max_by_key(|i| {
                Problem14::solve_given_robots_and_function(&mut robots.clone(), grid_width, grid_height, *i, most_robots_in_row)
            })
            .unwrap() as u128;

        let mut last_robots = robots.clone();
        Problem14::solve_given_robots(&mut last_robots, grid_width, grid_height, answer as isize);
        Self::display_robots(&last_robots, grid_width, grid_height, &self.tx);

        answer
    }
}

impl Problem14 {
    fn solve(input: &str, grid_width: isize, grid_height: isize, movements: isize) -> u128 {
        Self::solve_given_robots(&mut Self::parse(input), grid_width, grid_height, movements)
    }

    fn solve_given_robots(robots: &mut Vec<Robot>, grid_width: isize, grid_height: isize, movements: isize) -> u128 {
        let safety_factor_function = |robots: &Vec<Robot>, grid_width, grid_height| -> u128 {
            let upper_left_robots = robots
                .iter()
                .filter(|&robot| {
                    robot.position.0 < grid_width / 2 && robot.position.1 < grid_height / 2
                })
                .count();
            let lower_left_robots = robots
                .iter()
                .filter(|&robot| {
                    robot.position.0 < grid_width / 2 && robot.position.1 > grid_height / 2
                })
                .count();
            let lower_right_robots = robots
                .iter()
                .filter(|&robot| {
                    robot.position.0 > grid_width / 2 && robot.position.1 > grid_height / 2
                })
                .count();
            let upper_right_robots = robots
                .iter()
                .filter(|&robot| {
                    robot.position.0 > grid_width / 2 && robot.position.1 < grid_height / 2
                })
                .count();

            (upper_left_robots * lower_left_robots * lower_right_robots * upper_right_robots) as u128
        };
        Self::solve_given_robots_and_function(robots, grid_width, grid_height, movements, safety_factor_function)
    }

    fn solve_given_robots_and_function(robots: &mut Vec<Robot>, grid_width: isize, grid_height: isize, movements: isize, function: fn(&Vec<Robot>, isize, isize) -> u128) -> u128 {
        let robots_at_end = robots
            .into_iter()
            .map(|robot| {
                robot.mv(movements, grid_width, grid_height);

                robot.clone()
            })
            .collect_vec();

        function(&robots_at_end, grid_width, grid_height)
    }

    fn display_robots(robots: &[Robot], grid_width: isize, grid_height: isize, tx: &Sender<Event>) {
        let vofv = (0..grid_height).map(|row| {
            (0..grid_width).map(|col| {
                let count = robots.iter().filter(|&robot| robot.position.1 == row && robot.position.0 == col).count();
                let result = if count > 0 { '#' } else  { '.' };
                result
            })
                .collect_vec()
        }).collect_vec();

        let g = Grid::from_lines(vofv);

        tx.send(UpdateAppDisplayState(AppDisplayState::grid_update(g))).unwrap();
    }

    fn parse(input: &str) -> Vec<Robot> {
        let line_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

        input
            .lines()
            .map(|line| match line_regex.captures(line).unwrap().extract() {
                (_, [p_x, p_y, v_x, v_y]) => Robot {
                    position: (p_x.parse().unwrap(), p_y.parse().unwrap()),
                    velocity: (v_x.parse().unwrap(), v_y.parse().unwrap()),
                },
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_produce_correct_answer_for_part_1() {
        assert_eq!(
            Problem14::solve(
                "\
                p=0,4 v=3,-3\n\
                p=6,3 v=-1,-3\n\
                p=10,3 v=-1,2\n\
                p=2,0 v=2,-1\n\
                p=0,0 v=1,3\n\
                p=3,0 v=-2,-2\n\
                p=7,6 v=-1,-3\n\
                p=3,0 v=-1,-2\n\
                p=9,3 v=2,3\n\
                p=7,3 v=-1,2\n\
                p=2,4 v=2,-3\n\
                p=9,5 v=-3,-3\n",
                11,
                7,
                100
            ),
            12
        );
    }

    mod test_helpers {
        use super::*;

        mod test_mv {
            use super::*;

            #[test]
            fn should_move_correctly() {
                let mut robot = Robot {
                    position: (2, 4),
                    velocity: (2, -3),
                };
                let grid_height = 7;
                let grid_width = 11;
                robot.mv(1, grid_width, grid_height);
                assert_eq!(robot.position, (4, 1));
                robot.mv(1, grid_width, grid_height);
                assert_eq!(robot.position, (6, 5));
                robot.mv(1, grid_width, grid_height);
                assert_eq!(robot.position, (8, 2));
                robot.mv(1, grid_width, grid_height);
                assert_eq!(robot.position, (10, 6));
                robot.mv(1, grid_width, grid_height);
                assert_eq!(robot.position, (1, 3));
            }
        }
    }
}
