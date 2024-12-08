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
    type Item = ((i32, i32), char);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(col, &c)| ((row as i32, col as i32), c))
            })
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

    pub fn is_in_bounds(&self, point: &(i32, i32)) -> bool {
        point.0 >= 0
            && point.1 >= 0
            && point.0 < self.row_count as i32
            && point.1 < self.col_count as i32
    }
}
