use crate::problems::common::Grid;
use crate::problems::Problem;
use itertools::Itertools;
use std::collections::{BTreeSet, HashSet};

pub struct Problem12 {}

type Group = HashSet<((usize, usize), char)>;

trait PlantArea {
    fn get_perimeter(&self) -> u128;
    fn get_side_count(&self) -> u128;
    fn get_area(&self) -> u128;
    fn get_cost(&self) -> u128;
    fn get_bulk_cost(&self) -> u128;
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

    fn get_side_count(&self) -> u128 {
        let row_indices = self
            .iter()
            .map(|((i, _), _)| *i)
            .sorted()
            .dedup()
            .collect_vec();

        let row_edge_count = row_indices
            .iter()
            .map(|row| {
                let row_columns = self
                    .iter()
                    .filter_map(|&((i, j), _)| if i == *row { Some(j) } else { None })
                    .sorted()
                    .collect_vec();

                let above_columns = self
                    .iter()
                    .filter_map(|&((i, j), _)| {
                        if i == row.wrapping_sub(1) {
                            Some(j)
                        } else {
                            None
                        }
                    })
                    .sorted()
                    .collect_vec();

                let below_columns = self
                    .iter()
                    .filter_map(|&((i, j), _)| if i == row + 1 { Some(j) } else { None })
                    .sorted()
                    .collect_vec();

                // then same for columns left/right
                let edge_count_for_row = Problem12::find_edges(&row_columns, &above_columns)
                    + Problem12::find_edges(&row_columns, &below_columns);
                edge_count_for_row
            })
            .sum::<u128>();

        let col_indices = self
            .iter()
            .map(|((_, j), _)| *j)
            .sorted()
            .dedup()
            .collect_vec();

        let column_edge_count = col_indices
            .iter()
            .map(|col| {
                let col_rows = self
                    .iter()
                    .filter_map(|&((i, j), _)| if j == *col { Some(i) } else { None })
                    .sorted()
                    .collect_vec();

                let left_rows = self
                    .iter()
                    .filter_map(|&((i, j), _)| {
                        if j == col.wrapping_sub(1) {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .sorted()
                    .collect_vec();

                let right_rows = self
                    .iter()
                    .filter_map(|&((i, j), _)| if j == col + 1 { Some(i) } else { None })
                    .sorted()
                    .collect_vec();

                let edge_count_for_column = Problem12::find_edges(&col_rows, &left_rows)
                    + Problem12::find_edges(&col_rows, &right_rows);
                edge_count_for_column
            })
            .sum::<u128>();

        row_edge_count + column_edge_count
    }

    fn get_area(&self) -> u128 {
        self.len() as u128
    }

    fn get_cost(&self) -> u128 {
        let perimeter = self.get_perimeter();
        let area = self.get_area();
        perimeter * area
    }

    fn get_bulk_cost(&self) -> u128 {
        let side_count = self.get_side_count();
        let area = self.get_area();
        side_count * area
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
                    ((node.0 .0.wrapping_sub(1), node.0 .1), node.1),
                    ((node.0 .0 + 1, node.0 .1), node.1),
                    ((node.0 .0, node.0 .1.wrapping_sub(1)), node.1),
                    ((node.0 .0, node.0 .1 + 1), node.1),
                ]
                .into_iter()
                .filter(|n| unvisited_places.contains(n))
                .collect();

                nodes_to_visit.iter().for_each(|node| {
                    unvisited_places.remove(&node);
                });
                nodes_to_visit.extend(nodes_to_add);
            }

            result_vec.push(HashSet::from_iter(new_group));
        }

        result_vec
    }

    fn find_edges(line: &Vec<usize>, next_line: &Vec<usize>) -> u128 {
        if next_line == line {
            0 // entirely covered: 0 edges
        } else {
            let uncovered_items = line
                .iter()
                .filter(|&n| !next_line.contains(n))
                .sorted()
                .collect_vec();

            uncovered_items
                .iter()
                .fold((0u128, None), |(count_so_far, last_element), &element| {
                    if last_element.map(|l| l + 1) == Some(*element) {
                        (count_so_far, Some(*element))
                    } else {
                        (count_so_far + 1, Some(*element))
                    }
                })
                .0
        }
    }

    fn solve(&self, input: &str, g_f: fn(&Group) -> u128) -> u128 {
        let grid = Grid::from_string(input);

        let groups = Self::find_groups(&grid);

        groups.iter().map(|g| g_f(g)).sum()
    }
}

impl Problem<u128> for Problem12 {
    fn part1(&self, input: &str) -> u128 {
        let g_f = |g: &Group| g.get_cost();

        self.solve(input, g_f)
    }

    fn part2(&self, input: &str) -> u128 {
        let g_f = |g: &Group| g.get_bulk_cost();

        self.solve(input, g_f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_produce_correct_answer_for_part_1() {
        let p = Problem12 {};
        assert_eq!(
            p.part1(
                "\
            RRRRIICCFF\n\
            RRRRIICCCF\n\
            VVRRRCCFFF\n\
            VVRCCCJFFF\n\
            VVVVCJJCFE\n\
            VVIVCCJJEE\n\
            VVIIICJJEE\n\
            MIIIIIJJEE\n\
            MIIISIJEEE\n\
            MMMISSJEEE"
            ),
            1930
        );
    }

    #[test]
    fn should_produce_correct_answer_for_part_2() {
        let p = Problem12 {};
        assert_eq!(
            p.part2(
                "\
            RRRRIICCFF\n\
            RRRRIICCCF\n\
            VVRRRCCFFF\n\
            VVRCCCJFFF\n\
            VVVVCJJCFE\n\
            VVIVCCJJEE\n\
            VVIIICJJEE\n\
            MIIIIIJJEE\n\
            MIIISIJEEE\n\
            MMMISSJEEE"
            ),
            1206
        );
    }

    mod test_helpers {
        use super::*;
        use crate::problems::common::Grid;
        use itertools::Itertools;
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

            let costs = Problem12::find_groups(&g)
                .iter()
                .map(|g| g.get_cost())
                .collect_vec();
            assert_eq!(costs, vec![6 * 12 /*A*/, 4 * 10 /*B*/, 2 * 6 /*C*/])
        }

        mod test_edges {
            use super::*;

            fn get_a_group_from_grid(grid_string: &str) -> Group {
                let g: Grid<char> = Grid::from_string(grid_string);

                let groups = Problem12::find_groups(&g);

                groups
                    .into_iter()
                    .find(|g| g.iter().any(|x| (*x).1 == 'A'))
                    .unwrap()
            }

            #[test]
            fn should_find_correct_edge_count_on_simple_rectangles() {
                assert_eq!(
                    get_a_group_from_grid(
                        "\
                    AAAB\n\
                    AAAB\n\
                    BBBB",
                    )
                    .get_side_count(),
                    4
                );
                assert_eq!(get_a_group_from_grid("A").get_side_count(), 4);
                assert_eq!(
                    get_a_group_from_grid(
                        "\
                    AAAA\n\
                    AAAA\n\
                    AAAA",
                    )
                    .get_side_count(),
                    4
                );
                assert_eq!(
                    get_a_group_from_grid(
                        "\
                    CCCC\n\
                    CAAA\n\
                    CAAA",
                    )
                    .get_side_count(),
                    4
                );
                assert_eq!(
                    get_a_group_from_grid(
                        "\
                    CCCC\n\
                    CAAC\n\
                    CCCC",
                    )
                    .get_side_count(),
                    4
                );
            }

            #[test]
            fn should_handle_inner_gaps_correctly() {
                assert_eq!(
                    get_a_group_from_grid(
                        "\
                    AAA\n\
                    ABA\n\
                    AAA",
                    )
                    .get_side_count(),
                    8
                );
                assert_eq!(get_a_group_from_grid("A").get_side_count(), 4);
                assert_eq!(
                    get_a_group_from_grid(
                        "\
                    AAAA\n\
                    AABA\n\
                    ABAA\n\
                    AAAA",
                    )
                    .get_side_count(),
                    12
                );
                assert_eq!(
                    get_a_group_from_grid(
                        "\
                    AAAAAA\n\
                    AAABBA\n\
                    AAABBA\n\
                    ABBAAA\n\
                    ABBAAA\n\
                    AAAAAA"
                    )
                    .get_side_count(),
                    12
                )
            }

            #[test]
            fn should_handle_odd_shape() {
                assert_eq!(
                    get_a_group_from_grid(
                        "\
                    AAB\n\
                    CAA\n\
                    CCA",
                    )
                    .get_side_count(),
                    10
                );
                assert_eq!(
                    get_a_group_from_grid(
                        "\
                    AAAAA\n\
                    AXXXX\n\
                    AAAAA\n\
                    AXXXX\n\
                    AAAAA",
                    )
                    .get_side_count(),
                    12
                );
            }
        }
    }
}
