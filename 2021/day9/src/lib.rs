use std::str::FromStr;

#[derive(Debug)]
struct Heightmap {
    data: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl FromStr for Heightmap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<_> = s.trim().split('\n').collect::<Vec<&str>>();
        let mut data: Vec<Vec<u8>> = vec![];
        for row in &rows {
            data.push(
                row.chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>(),
            );
        }
        Ok(Heightmap {
            data,
            rows: rows.len(),
            cols: rows[0].chars().count(),
        })
    }
}

impl Heightmap {
    fn resolve(&self) -> usize {
        let mut count: usize = 0;

        for i in 0..self.rows {
            for j in 0..self.cols {
                let top = if i > 0 { self.data[i - 1][j] } else { 10 };

                let bottom = if i < self.rows - 1 {
                    self.data[i + 1][j]
                } else {
                    10
                };

                let left = if j > 0 { self.data[i][j - 1] } else { 10 };

                let right = if j < self.cols - 1 {
                    self.data[i][j + 1]
                } else {
                    10
                };
                let center = self.data[i][j];

                if center < top && center < bottom && center < left && center < right {
                    count += (center + 1) as usize;
                }
            }
        }

        count
    }
}

pub fn part1(input: &str) -> usize {
    let grid: Heightmap = input.parse().unwrap();
    let result = grid.resolve();

    result
}

#[cfg(test)]
mod day9_test {
    use super::*;

    #[test]
    fn test_with_example_data() {
        let data = include_str!("../example.txt");
        let result = part1(data);

        assert_eq!(result, 15);
    }

    #[test]
    fn test_puzzle_input_part1() {
        let data = include_str!("../input.txt");
        let result = part1(data);

        assert_eq!(result, 541);
    }
}
