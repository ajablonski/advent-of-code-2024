use crate::problems::common::Grid;
use crate::problems::Problem;
use clap::builder::TypedValueParser;
use std::collections::{BTreeSet, HashSet};

pub struct Problem12 {}

type Group = HashSet<((usize, usize), char)>;

trait PlantArea {
    fn get_perimeter(&self) -> u128;
    fn get_area(&self) -> u128;
    fn get_cost(&self) -> u128;
}

impl PlantArea for Group {
    fn get_perimeter(&self) -> u128 {
        self.iter()
            .map(|((i, j), c)| {
                let mut outside_edges = 0;
                if !self.contains(&((i + 1, *j), *c)) {
                    outside_edges += 1;
                }
                if !self.contains(&((*i, j + 1), *c)) {
                    outside_edges += 1;
                }
                if !self.contains(&((i.wrapping_sub(1), *j), *c)) {
                    outside_edges += 1;
                }
                if !self.contains(&((*i, j.wrapping_sub(1)), *c)) {
                    outside_edges += 1;
                }

                outside_edges
            })
            .sum()
    }

    fn get_area(&self) -> u128 {
        self.len() as u128
    }

    fn get_cost(&self) -> u128 {
        let perimeter = self.get_perimeter();
        let area = self.get_area();
        perimeter * area
    }
}
impl Problem12 {
    pub fn find_groups(grid: &Grid<char>) -> Vec<Group> {
        let mut unvisited_places: BTreeSet<((usize, usize), char)> = BTreeSet::from_iter(
            grid.clone()
                .into_iter()
                .map(|((r, c), ch)| ((r as usize, c as usize), ch)),
        );

        let mut result_vec = vec![];

        while !unvisited_places.is_empty() {
            let mut new_group: Vec<((usize, usize), char)> = vec![];

            let mut nodes_to_visit = vec![unvisited_places.pop_first().unwrap()];

            while !nodes_to_visit.is_empty() {
                let node = nodes_to_visit.pop().unwrap();

                new_group.push(node);
                unvisited_places.remove(&node);

                let nodes_to_add: Vec<((usize, usize), char)> = vec![
                    ((node.0.0.wrapping_sub(1), node.0.1), node.1),
                    ((node.0.0 + 1, node.0.1), node.1),
                    ((node.0.0, node.0.1.wrapping_sub(1)), node.1),
                    ((node.0.0, node.0.1 + 1), node.1),
                ]
                    .into_iter()
                    .filter(|n| unvisited_places.contains(n))
                    .collect();

                nodes_to_visit.iter().for_each(|node| {unvisited_places.remove(&node);});
                nodes_to_visit.extend(nodes_to_add);

            }

            result_vec.push(HashSet::from_iter(new_group));


        }

        result_vec
    }

    fn solve(&self, input: &str) -> u128 {
        let grid = Grid::from_string(input);

        let groups = Self::find_groups(&grid);

        groups.iter().map(|g| g.get_cost()).sum()
    }
}

impl Problem<u128> for Problem12 {
    fn part1(&self, input: &str) -> u128 {
        self.solve(input)
    }

    fn part2(&self, _input: &str) -> u128 {
        12
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_produce_correct_answer_for_part_1() {
        let p = Problem12 {};
        assert_eq!(p.part1("\
            RRRRIICCFF\n\
            RRRRIICCCF\n\
            VVRRRCCFFF\n\
            VVRCCCJFFF\n\
            VVVVCJJCFE\n\
            VVIVCCJJEE\n\
            VVIIICJJEE\n\
            MIIIIIJJEE\n\
            MIIISIJEEE\n\
            MMMISSJEEE"), 1930);
    }

    #[test]
    fn should_produce_correct_answer_for_part_2() {
        let p = Problem12 {};
        assert_eq!(p.part2(""), 0);
    }

    mod test_helpers {
        use super::*;
        use crate::problems::common::Grid;
        use std::collections::HashSet;

        #[test]
        fn find_groups_should_find_correct_plant_groups() {
            let g: Grid<char> = Grid::from_string(
                "\
            AAAB\n\
            AABB\n\
            ACCB\n
            ",
            );

            assert_eq!(
                Problem12::find_groups(&g),
                vec![
                    HashSet::from([
                        ((0, 0), 'A'),
                        ((0, 1), 'A'),
                        ((0, 2), 'A'),
                        ((1, 0), 'A'),
                        ((1, 1), 'A'),
                        ((2, 0), 'A'),
                    ]),
                    HashSet::from([((0, 3), 'B'), ((1, 2), 'B'), ((1, 3), 'B'), ((2, 3), 'B'),]),
                    HashSet::from([((2, 1), 'C'), ((2, 2), 'C')])
                ]
            )
        }

        #[test]
        fn find_group_cost_should_find_correct_cost_for_a_group() {
            let g: Grid<char> = Grid::from_string(
                "\
            AAAB\n\
            AABB\n\
            ACCB\n
            ",
            );

            // Test cases on left/top border of grid
            // Test cases on right/bottom border of grid
            // Test cases in center of grid
            // Test cases where there is an internal border (one group around another)
        }
    }
}
