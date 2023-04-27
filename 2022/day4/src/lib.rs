use std::cmp::{max, min};
use std::convert::From;

#[derive(Debug)]
struct Range {
    start: i16,
    end: i16,
}

#[derive(Debug)]
struct Ranges {
    one: Range,
    two: Range,
}

impl From<&str> for Range {
    fn from(item: &str) -> Self {
        let dash_idx = item.find("-").expect("Can't find dash, can't parse.");
        let start: i16 = item[..dash_idx].parse().unwrap();
        let end: i16 = item[dash_idx + 1..].parse().unwrap();

        Range { start, end }
    }
}

impl From<&str> for Ranges {
    fn from(item: &str) -> Self {
        let comma_idx = item.find(",").expect("Comma not found, can't parse.");

        Ranges {
            one: item[..comma_idx].into(),
            two: item[comma_idx + 1..].into(),
        }
    }
}

impl Ranges {
    fn are_subset(&self) -> bool {
        let a = &self.one;
        let b = &self.two;

        if a.start <= b.start && a.end >= b.end {
            return true;
        }

        if b.start <= a.start && b.end >= a.end {
            return true;
        }

        false
    }

    fn are_overlap(&self) -> bool {
        let a = &self.one;
        let b = &self.two;

        return min(a.end, b.end) - max(a.start, b.start) >= 0;
    }
}

pub fn part1(input: &str) -> u32 {
    let ranges: Vec<Ranges> = input.trim().lines().map(|x| x.into()).collect();
    let mut tot: u32 = 0;

    for range in ranges {
        if range.are_subset() {
            tot += 1;
        }
    }

    tot
}

pub fn part2(input: &str) -> u32 {
    let ranges: Vec<Ranges> = input.trim().lines().map(|x| x.into()).collect();
    let mut tot: u32 = 0;

    for range in ranges {
        if range.are_subset() || range.are_overlap() {
            tot += 1;
        }
    }

    tot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let data = include_str!("../example.txt");

        let result = part1(data);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_input1() {
        let data = include_str!("../input.txt");

        let result = part1(data);

        assert_eq!(result, 595);
    }

    #[test]
    fn test_example2() {
        let data = include_str!("../example.txt");

        let result = part2(data);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_input2() {
        let data = include_str!("../input.txt");

        let result = part2(data);

        assert_eq!(result, 952);
    }
}
