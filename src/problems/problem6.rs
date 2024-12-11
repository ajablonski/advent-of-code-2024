use crate::problems::common::Grid;
use crate::problems::problem6::Direction::{EAST, NORTH, SOUTH, WEST};
use crate::problems::Problem;
use crate::Event;
use std::collections::HashSet;
use std::sync::mpsc;

pub struct Problem6 {
    _tx: mpsc::Sender<Event>,
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Clone)]
struct GuardGrid {
    grid: Grid<char>,
    guard_position: (i32, i32),
    guard_direction: Direction,
    obstacle_positions: HashSet<(i32, i32)>,
    visited_positions: HashSet<(i32, i32)>,
    has_guard: bool,
    has_looped: bool,
    visited_states: HashSet<(i32, i32, Direction)>,
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
        let (guard_position, guard_direction) = maybe_guard_details.get(0).unwrap();
        let visited_positions: HashSet<(i32, i32)> = HashSet::from([guard_position.clone()]);
        let visited_states: HashSet<(i32, i32, Direction)> = HashSet::from([(
            guard_position.0.clone(),
            guard_position.1.clone(),
            guard_direction.clone(),
        )]);

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
            grid: Grid {
                lines: vec![],
                row_count: grid.row_count,
                col_count: grid.col_count,
            },
            guard_position: guard_position.clone(),
            guard_direction: guard_direction.clone(),
            obstacle_positions,
            visited_positions,
            has_guard: true,
            has_looped: false,
            visited_states,
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
        let mut visited_states = self.visited_states.clone();
        let new_state = (guard_position.0, guard_position.1, guard_direction.clone());
        let has_looped = self.visited_states.contains(&new_state);

        if has_guard {
            visited_positions.insert(guard_position);
            visited_states.insert(new_state);
        }

        GuardGrid {
            guard_direction,
            guard_position,
            visited_positions,
            has_guard,
            grid: self.grid.clone(),
            visited_states,
            has_looped,
            obstacle_positions: self.obstacle_positions.clone()
        }
    }

    fn advance_all(& self) -> GuardGrid {
        let mut grid = self.clone();

        while grid.has_guard && !grid.has_looped {
            grid = grid.advance();
        }

        grid.clone()
    }
}

impl Problem<u128> for Problem6 {
    fn part1(&self, input: &str) -> u128 {
        GuardGrid::from_string(input)
            .advance_all()
            .visited_positions
            .len() as u128
    }

    fn part2(&self, input: &str) -> u128 {
        let original_grid = GuardGrid::from_string(input);
        let visited_positions = original_grid.advance_all().visited_positions;

        visited_positions
            .iter()
            .filter(|&visited_position| {
                println!("visited position: {:?}", visited_position);

                let mut new_obstacle_positions = original_grid.obstacle_positions.clone();
                new_obstacle_positions.insert(*visited_position);
                let grid_with_additional_obstacle = GuardGrid {
                    obstacle_positions: new_obstacle_positions,
                    ..original_grid.clone()
                };

                grid_with_additional_obstacle.advance_all().has_looped
            })
            .count() as u128
    }
}

impl Problem6 {
    pub fn new(tx: &mpsc::Sender<Event>) -> Problem6 {
        Problem6 { _tx: tx.clone() }
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

        assert_eq!(
            p.part2(
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
            6
        );
    }
}
