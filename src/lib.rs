pub trait Problem {
    fn part1(&self, input: &str) -> u32;

    fn part2(&self, input: &str) -> u32;
}

pub mod problem1;
