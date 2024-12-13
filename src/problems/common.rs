use std::fmt;
use std::fmt::Write;

#[derive(Clone)]
pub struct Grid<T>
where
    T: FromChar<T>,
{
    pub lines: Vec<Vec<T>>,
    pub row_count: usize,
    pub col_count: usize,
}

pub trait FromChar<T> {
    fn from_char(c: char) -> Option<T>;
}

impl FromChar<char> for char {
    fn from_char(c: char) -> Option<char> {
        Some(c)
    }
}

impl FromChar<u32> for u32 {
    fn from_char(c: char) -> Option<u32> {
        c.to_digit(10)
    }
}

impl fmt::Debug for Grid<char> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('\n')?;
        self.lines.iter().for_each(|line| {
            line.iter().for_each(|c| f.write_char(*c).unwrap());

            f.write_char('\n').unwrap();
        });

        Ok(())
    }
}

impl<T> IntoIterator for Grid<T>
where
    T: FromChar<T> + Clone
{
    type Item = ((i32, i32), T);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(col, &ref c)| ((row as i32, col as i32), c.clone()))
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<T> Grid<T>
where
    T: FromChar<T>,
{
    fn from_lines(lines: Vec<Vec<T>>) -> Self {
        Self {
            row_count: lines.len(),
            col_count: lines[0].len(),
            lines,
        }
    }

    pub fn from_string (input: &str) -> Self {
        let lines = input
            .lines()
            .map(|l| {
                l.chars()
                    .filter(|c| !c.is_whitespace())
                    .flat_map(|c| <T as FromChar<T>>::from_char(c))
                    .collect::<Vec<T>>()
            })
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
