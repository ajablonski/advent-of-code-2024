use crate::problems::common::Grid;
use crate::problems::Problem;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Problem8 {}

impl Problem<u128> for Problem8 {
    fn part1(&self, input: &str) -> u128 {
        let grid = Grid::from_string(input);

        let frequencies_and_locations = Problem8::find_frequencies_and_locations(&grid);

        frequencies_and_locations
            .iter()
            .flat_map(|(_, freq)| Problem8::find_pairs(freq))
            .flat_map(|pair| Problem8::find_antinodes(pair))
            .filter(|antinode| {
                antinode.0 >= 0
                    && antinode.1 >= 0
                    && antinode.0 < grid.row_count as i32
                    && antinode.1 < grid.col_count as i32
            })
            .sorted()
            .dedup()
            .count() as u128
    }

    fn part2(&self, input: &str) -> u128 {
        let grid = Grid::from_string(input);

        let frequencies_and_locations = Problem8::find_frequencies_and_locations(&grid);

        frequencies_and_locations
            .iter()
            .flat_map(|(_, freq)| Problem8::find_pairs(freq))
            .flat_map(|pair| Problem8::find_harmonic_antinodes(pair, grid.row_count, grid.col_count))
            .sorted()
            .dedup()
            .count() as u128
    }
}

impl Problem8 {
    fn find_frequencies_and_locations(grid: &Grid) -> HashMap<char, Vec<(i32, i32)>> {
        grid.clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, cell| {
                if cell.1 != '.' {
                    let cell_as_i32 = (cell.0 .0 as i32, cell.0 .1 as i32);
                    acc.entry(cell.1)
                        .and_modify(|v| v.push(cell_as_i32))
                        .or_insert(vec![cell_as_i32]);
                }
                acc
            })
    }

    fn find_pairs(locations: &Vec<(i32, i32)>) -> Vec<((i32, i32), (i32, i32))> {
        locations
            .iter()
            .combinations(2)
            .map(|combo| (*combo[0], *combo[1]))
            .collect_vec()
    }

    fn find_antinodes(node_pair: ((i32, i32), (i32, i32))) -> HashSet<(i32, i32)> {
        let row_difference = node_pair.1 .0 - node_pair.0 .0;

        let col_difference = node_pair.1 .1 - node_pair.0 .1;

        HashSet::from([
            (
                node_pair.0 .0 - row_difference,
                node_pair.0 .1 - col_difference,
            ),
            (
                node_pair.1 .0 + row_difference,
                node_pair.1 .1 + col_difference,
            ),
        ])
    }

    fn find_harmonic_antinodes(
        node_pair: ((i32, i32), (i32, i32)),
        row_count: usize,
        col_count: usize,
    ) -> HashSet<(i32, i32)> {
        let row_difference = node_pair.1 .0 - node_pair.0 .0;

        let col_difference = node_pair.1 .1 - node_pair.0 .1;

        let is_in_range = |location: &(i32, i32)| -> bool {
            location.0 >= 0
                && location.1 >= 0
                && location.0 < row_count as i32
                && location.1 < col_count as i32
        };


        let decreasing_iterator = (0..)
            .map(|harmonic_count| {
                (
                    node_pair.0 .0 - harmonic_count * row_difference,
                    node_pair.0 .1 - harmonic_count * col_difference,
                )
            })
            .take_while(is_in_range);

        let increasing_iterator = (0..)
            .map(|harmonic_count| {
                (
                    node_pair.0 .0 + harmonic_count * row_difference,
                    node_pair.0 .1 + harmonic_count * col_difference,
                )
            })
            .take_while(is_in_range);

        decreasing_iterator.chain(increasing_iterator).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_produce_correct_answer_for_part_1() {
        let p = Problem8 {};
        assert_eq!(
            p.part1(
                "\
                ............\n\
                ........0...\n\
                .....0......\n\
                .......0....\n\
                ....0.......\n\
                ......A.....\n\
                ............\n\
                ............\n\
                ........A...\n\
                .........A..\n\
                ............\n\
                ............"
            ),
            14
        );
    }

    #[test]
    fn should_produce_correct_answer_for_part_2() {
        let p = Problem8 {};
        assert_eq!(
            p.part2(
                "\
                ............\n\
                ........0...\n\
                .....0......\n\
                .......0....\n\
                ....0.......\n\
                ......A.....\n\
                ............\n\
                ............\n\
                ........A...\n\
                .........A..\n\
                ............\n\
                ............"
            ),
            34
        );
    }

    mod test_helper_functions {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn find_frequencies_should_list_available_frequencies() {
            let grid = Grid::from_string(
                "\
            ...A...\n\
            .B.B...\n\
            .......",
            );

            let expected = HashMap::from([('A', vec![(0, 3)]), ('B', vec![(1, 1), (1, 3)])]);
            assert_eq!(Problem8::find_frequencies_and_locations(&grid), expected);
        }

        #[test]
        fn find_pairs_should_find_all_possible_combinations_of_two_frequency_locations() {
            assert_eq!(Problem8::find_pairs(&vec![]), vec![]);
            assert_eq!(Problem8::find_pairs(&vec![(0, 0)]), vec![]);
            assert_eq!(
                Problem8::find_pairs(&vec![(0, 0), (2, 2)]),
                vec![((0, 0), (2, 2))]
            );
            assert_eq!(
                Problem8::find_pairs(&vec![(0, 0), (2, 2), (4, 4)]),
                vec![((0, 0), (2, 2)), ((0, 0), (4, 4)), ((2, 2), (4, 4)),]
            );
            assert_eq!(
                Problem8::find_pairs(&vec![(0, 0), (2, 2), (4, 4), (6, 6)]),
                vec![
                    ((0, 0), (2, 2)),
                    ((0, 0), (4, 4)),
                    ((0, 0), (6, 6)),
                    ((2, 2), (4, 4)),
                    ((2, 2), (6, 6)),
                    ((4, 4), (6, 6)),
                ]
            );
        }

        #[test]
        fn find_antinodes_from_pair_should_list_all_possible_antinodes() {
            let vertical_pair = ((0, 0), (2, 0));
            let vertical_pair_reversed = ((2, 0), (0, 0));
            let horizontal_pair = ((0, 0), (0, 2));
            let horizontal_pair_reversed = ((0, 2), (0, 0));
            let diagonal_pair = ((0, 0), (3, 2));
            let diagonal_pair_reversed = ((3, 2), (0, 0));
            assert_eq!(
                Problem8::find_antinodes(vertical_pair),
                HashSet::from([(-2, 0), (4, 0)])
            );
            assert_eq!(
                Problem8::find_antinodes(vertical_pair_reversed),
                HashSet::from([(-2, 0), (4, 0)])
            );
            assert_eq!(
                Problem8::find_antinodes(horizontal_pair),
                HashSet::from([(0, -2), (0, 4)])
            );
            assert_eq!(
                Problem8::find_antinodes(horizontal_pair_reversed),
                HashSet::from([(0, -2), (0, 4)])
            );
            assert_eq!(
                Problem8::find_antinodes(diagonal_pair),
                HashSet::from([(-3, -2), (6, 4)])
            );
            assert_eq!(
                Problem8::find_antinodes(diagonal_pair_reversed),
                HashSet::from([(-3, -2), (6, 4)])
            );
        }

        #[test]
        fn find_harmonic_antinodes_should_find_all_antinodes() {
            let horizontal_pair = ((0, 4), (0, 6));

            assert_eq!(
                Problem8::find_harmonic_antinodes(horizontal_pair, 10, 10),
                HashSet::from([(0, 4), (0, 2), (0, 0), (0, 6), (0, 8)])
            )
        }
    }
}
