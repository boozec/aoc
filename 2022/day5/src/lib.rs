use std::collections::HashMap;
use std::collections::VecDeque;

pub fn part1(input: &str) -> String {
    let mut h: Vec<VecDeque<char>> = Vec::with_capacity(10);
    h.resize(10, VecDeque::new());

    let split: Vec<&str> = input.split("\n\n").collect();
    let mut m: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;
    let mut j = 1;
    while i < 10 {
        m.insert(j, i);
        j += 4;
        i += 1;
    }

    assert!(split.len() == 2);

    for line in split[0].lines() {
        if line.chars().nth(1).unwrap() == '1' {
            continue;
        }

        for (i, ch) in line.chars().enumerate() {
            match ch {
                ' ' | '[' | ']' => {
                    continue;
                }
                x => {
                    let idx = m[&i];
                    h[idx].push_front(x);
                }
            };
        }
    }

    for line in split[1].lines() {
        let actions: Vec<usize> = line
            .split(' ')
            .enumerate()
            .filter(|&(i, _)| i % 2 == 1)
            .map(|(_, e)| e.parse::<usize>().unwrap())
            .collect();

        for _ in 0..actions[0] {
            let x = h[actions[1] - 1].pop_back().unwrap();
            h[actions[2] - 1].push_back(x);
        }
    }

    let mut s = String::new();

    for mut e in h {
        if e.len() == 0 {
            continue;
        }

        let x = e.pop_back().unwrap().to_string();
        s += &x;
    }

    s
}

pub fn part2(input: &str) -> String {
    let mut h: Vec<VecDeque<char>> = Vec::with_capacity(10);
    h.resize(10, VecDeque::new());

    let split: Vec<&str> = input.split("\n\n").collect();
    let mut m: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;
    let mut j = 1;
    while i < 10 {
        m.insert(j, i);
        j += 4;
        i += 1;
    }

    assert!(split.len() == 2);

    for line in split[0].lines() {
        if line.chars().nth(1).unwrap() == '1' {
            continue;
        }

        for (i, ch) in line.chars().enumerate() {
            match ch {
                ' ' | '[' | ']' => {
                    continue;
                }
                x => {
                    let idx = m[&i];
                    h[idx].push_front(x);
                }
            };
        }
    }

    for line in split[1].lines() {
        let actions: Vec<usize> = line
            .split(' ')
            .enumerate()
            .filter(|&(i, _)| i % 2 == 1)
            .map(|(_, e)| e.parse::<usize>().unwrap())
            .collect();

        let mut xs = Vec::<char>::new();
        for _ in 0..actions[0] {
            xs.push(h[actions[1] - 1].pop_back().unwrap());
        }
        xs.reverse();
        for x in xs {
            h[actions[2] - 1].push_back(x);
        }
    }

    let mut s = String::new();

    for mut e in h {
        if e.len() == 0 {
            continue;
        }

        let x = e.pop_back().unwrap().to_string();
        s += &x;
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");

        assert_eq!(part1(input), String::from("CMZ"));
    }

    #[test]
    fn test_input1() {
        let input = include_str!("../input.txt");

        assert_eq!(part1(input), String::from("RTGWZTHLD"));
    }

    #[test]
    fn test_example2() {
        let input = include_str!("../example.txt");

        assert_eq!(part2(input), String::from("MCD"));
    }

    #[test]
    fn test_input2() {
        let input = include_str!("../input.txt");

        assert_eq!(part2(input), String::from("STHGRZZFR"));
    }
}
