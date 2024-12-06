use crate::problems::common::Grid;
use crate::problems::problem6::Direction::{EAST, NORTH, SOUTH, WEST};
use crate::problems::Problem;
use std::collections::HashSet;
use std::sync::mpsc;
use crate::Event;

pub struct Problem6 {
    tx: mpsc::Sender<Event>
}

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Clone)]
struct GuardGrid {
    grid: Grid,
    guard_position: (i32, i32),
    guard_direction: Direction,
    obstacle_positions: HashSet<(i32, i32)>,
    visited_positions: HashSet<(i32, i32)>,
    has_guard: bool,
}

impl GuardGrid {
    fn from_string(s: &str) -> GuardGrid {
        let grid = Grid::from_string(s);

        let maybe_guard_details = grid
            .lines
            .iter()
            .enumerate()
            .flat_map(|(row_num, row)| {
                row.iter().enumerate().filter_map(move |(col_num, &c)| {
                    let pos = (row_num as i32, col_num as i32);
                    match c {
                        '^' => Some((pos, NORTH)),
                        '>' => Some((pos, EAST)),
                        'V' => Some((pos, SOUTH)),
                        '<' => Some((pos, WEST)),
                        _ => None,
                    }
                })
            })
            .collect::<Vec<((i32, i32), Direction)>>();
        let (guard_position, guard_direction) = maybe_guard_details
            .get(0)
            .unwrap();
        let visited_positions: HashSet<(i32, i32)> = HashSet::from([guard_position.clone()]);

        let obstacle_positions = grid
            .lines
            .iter()
            .enumerate()
            .flat_map(|(row_num, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(col_num, &c)| match c {
                        '#' => Some((row_num as i32, col_num as i32)),
                        _ => None,
                    })
                    .collect::<Vec<(i32, i32)>>()
            })
            .collect::<HashSet<(i32, i32)>>();

        GuardGrid {
            grid,
            guard_position: guard_position.clone(),
            guard_direction: guard_direction.clone(),
            obstacle_positions,
            visited_positions,
            has_guard: true,
        }
    }

    fn advance(&self) -> GuardGrid {
        let potential_next_position = match self.guard_direction {
            NORTH => (self.guard_position.0 - 1, self.guard_position.1),
            EAST => (self.guard_position.0, self.guard_position.1 + 1),
            SOUTH => (self.guard_position.0 + 1, self.guard_position.1),
            WEST => (self.guard_position.0, self.guard_position.1 - 1),
        };

        let guard_direction = match self.guard_direction {
            NORTH => {
                if self.obstacle_positions.contains(&potential_next_position) {
                    EAST
                } else {
                    NORTH
                }
            }
            EAST => {
                if self.obstacle_positions.contains(&potential_next_position) {
                    SOUTH
                } else {
                    EAST
                }
            }
            SOUTH => {
                if self.obstacle_positions.contains(&potential_next_position) {
                    WEST
                } else {
                    SOUTH
                }
            }
            WEST => {
                if self.obstacle_positions.contains(&potential_next_position) {
                    NORTH
                } else {
                    WEST
                }
            }
        };

        let guard_position = if guard_direction == self.guard_direction {
            potential_next_position
        } else {
            self.guard_position
        };



        let has_guard = self.has_guard
            && guard_position.0 >= 0
            && guard_position.0 < self.grid.row_count as i32
            && guard_position.1 >= 0
            && guard_position.1 < self.grid.col_count as i32;
        let mut visited_positions = self.visited_positions.clone();

        if has_guard {
            visited_positions.insert(guard_position);
        }



        let grid = self.grid.lines
            .iter()
            .enumerate()
            .map(|(row_num, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_num, &c)| {
                        let char_position = (row_num as i32, col_num as i32);
                        if char_position == guard_position {
                            match guard_direction {
                                NORTH => '^',
                                EAST => '>',
                                SOUTH => 'V',
                                WEST => '<',
                            }
                        } else if char_position == self.guard_position {
                            'X'
                        } else {
                            c
                        }
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        GuardGrid {
            guard_direction,
            guard_position,
            visited_positions,
            has_guard,
            grid: Grid {
                row_count: self.grid.row_count,
                col_count: self.grid.col_count,
                lines: grid
            },
            ..self.clone()
        }
    }
}

impl Problem<u128> for Problem6 {
    fn part1(&self, input: &str) -> u128 {
        let mut grid = GuardGrid::from_string(input);

        while grid.has_guard {
            grid = grid.advance();
        }

        grid.visited_positions.len() as u128
    }

    fn part2(&self, _input: &str) -> u128 {
        0
    }
}

impl Problem6 {
    pub fn new(tx: &mpsc::Sender<Event>) -> Problem6 {
        Problem6 { tx: tx.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_produce_correct_answer_for_part_1() {
        let p: Problem6 = Problem6::new(&mpsc::channel().0);

        assert_eq!(
            p.part1(
                "\
        ....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#..."
            ),
            41
        );
    }

    #[test]
    fn should_produce_correct_answer_for_part_2() {
        let p: Problem6 = Problem6::new(&mpsc::channel().0);

        assert_eq!(p.part2(""), 0);
    }
}
