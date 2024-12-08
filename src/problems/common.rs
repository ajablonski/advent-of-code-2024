use std::fmt;
use std::fmt::Write;

#[derive(Clone)]
pub struct Grid {
    pub lines: Vec<Vec<char>>,
    pub row_count: usize,
    pub col_count: usize,
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

impl IntoIterator for Grid {
    type Item = ((usize, usize), char);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(row, line)| line.iter().enumerate().map(move |(col, &c)| ((row, col), c)))
            .collect::<Vec<_>>()
            .into_iter()
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

    pub fn from_string(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();

        Self::from_lines(lines)
    }
}
