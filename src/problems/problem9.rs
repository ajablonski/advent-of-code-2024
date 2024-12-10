use crate::problems::Problem;
use itertools::Itertools;
use std::ops::Range;

pub struct Problem9 {}

#[derive(PartialEq, Clone, Debug)]
struct File {
    id: u64,
    locations: Vec<u64>,
}

#[derive(PartialEq, Clone, Debug)]
struct FileSystem {
    pub files: Vec<File>,
    pub free_spaces: Vec<Range<u64>>,
}

impl FileSystem {
    pub(crate) fn compact(&mut self) -> FileSystem {
        let free_space_iterator = self.free_spaces.clone().into_iter();
        let mut free_spaces = vec![];
        for range in free_space_iterator {
            free_spaces.extend(range);
        }

        let mut last_file_iter = self.files.iter_mut().rev();
        let mut last_file = last_file_iter.next().unwrap();
        let mut last_file_locations = last_file.locations.clone();
        let mut new_locations = vec![];

        for free_location in free_spaces {
            let location_to_replace = last_file_locations.pop().unwrap();
            if location_to_replace > free_location {
                new_locations.push(free_location);

                if last_file_locations.is_empty() {
                    last_file.locations = new_locations;
                    println!("Done with");

                    // move to next file
                    last_file = last_file_iter.next().unwrap();
                    last_file_locations = last_file.locations.clone();
                    new_locations = vec![];
                }
            } else {
                println!("ENDING");
                new_locations.extend(last_file_locations);
                new_locations.push(location_to_replace);
                last_file.locations = new_locations;

                break;
            }
        }

        self.clone()
    }
}

impl Problem<u128> for Problem9 {
    fn part1(&self, input: &str) -> u128 {
        let mut file_system = Problem9::parse(input);

        file_system
            .compact()
            .files
            .iter()
            .map(|file| {
                file.clone()
                    .locations
                    .into_iter()
                    .map(|location| location * file.id)
                    .sum::<u64>()
            })
            .sum::<u64>() as u128
    }

    fn part2(&self, input: &str) -> u128 {
        0
    }
}

impl Problem9 {
    fn parse(input: &str) -> FileSystem {
        input
            .char_indices()
            .filter(|(_, c)| *c != '\n')
            .fold(
                (
                    FileSystem {
                        files: vec![],
                        free_spaces: vec![],
                    },
                    0u64,
                ),
                |(mut fs, fs_index), (i, c)| {
                    let size = c.to_digit(10).unwrap() as u64;
                    let end_index = fs_index + size;
                    if i % 2 == 0 {
                        fs.files.push(File {
                            id: i as u64 / 2,
                            locations: (fs_index..end_index).collect_vec(),
                        });
                        (fs, end_index)
                    } else {
                        fs.free_spaces.push(fs_index..end_index);
                        (fs, end_index)
                    }
                },
            )
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_produce_correct_answer_for_part_1() {
        let p = Problem9 {};
        assert_eq!(p.part1("2333133121414131402"), 1928);
    }

    #[test]
    fn should_produce_correct_answer_for_part_2() {
        let p = Problem9 {};
        assert_eq!(p.part2(""), 0);
    }
    // 00...111...2...333.44.5555.6666.777.888899
    mod test_helper_functions {
        use super::*;

        #[test]
        fn should_parse_input() {
            assert_eq!(
                Problem9::parse("12345\n"),
                FileSystem {
                    files: vec![
                        File {
                            id: 0,
                            locations: vec![0]
                        },
                        File {
                            id: 1,
                            locations: vec![3, 4, 5]
                        },
                        File {
                            id: 2,
                            locations: vec![10, 11, 12, 13, 14]
                        },
                    ],
                    free_spaces: vec![1..3, 6..10],
                }
            )
        }

        #[test]
        fn should_compact_filesystem() {
            assert_eq!(
                FileSystem {
                    files: vec![
                        File {
                            id: 0,
                            locations: vec![0]
                        },
                        File {
                            id: 1,
                            locations: vec![3, 4, 5]
                        },
                        File {
                            id: 2,
                            locations: vec![10, 11, 12, 13, 14]
                        },
                    ],
                    free_spaces: vec![1..3, 6..10],
                }
                .compact()
                .files,
                vec![
                    File {
                        id: 0,
                        locations: vec![0]
                    },
                    File {
                        id: 1,
                        locations: vec![3, 4, 5]
                    },
                    File {
                        id: 2,
                        locations: vec![1, 2, 6, 7, 8]
                    },
                ]
            )
        }
    }
}
