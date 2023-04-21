use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> usize {
    let mut tot: usize = 0;

    let lines = input
        .lines()
        .map(|x| String::from(x))
        .collect::<Vec<String>>();

    let mut map = HashMap::<char, usize>::new();

    for (i, ch) in ('a'..='z').enumerate() {
        map.insert(ch, i + 1);
    }
    for (i, ch) in ('A'..='Z').enumerate() {
        map.insert(ch, i + 27);
    }

    for line in lines {
        let half = line.len() / 2;

        let (l, r) = (&line[0..half], &line[half..]);

        'o: for ch in l.chars() {
            for ch2 in r.chars() {
                if ch == ch2 {
                    tot += map[&ch];
                    break 'o;
                }
            }
        }
    }

    tot
}

pub fn part2(input: &str) -> usize {
    let mut tot: usize = 0;

    let lines = input
        .lines()
        .map(|x| String::from(x))
        .collect::<Vec<String>>();

    let mut map = HashMap::<char, usize>::new();

    for (i, ch) in ('a'..='z').enumerate() {
        map.insert(ch, i + 1);
    }
    for (i, ch) in ('A'..='Z').enumerate() {
        map.insert(ch, i + 27);
    }

    let n = lines.len() / 3;
    let mut group_of_3 = Vec::<Vec<String>>::with_capacity(n);
    for _ in 0..n {
        group_of_3.push(vec![]);
    }

    let mut idx = 0;

    for (i, line) in lines.iter().enumerate() {
        if i % 3 == 0 && i != 0 {
            idx += 1;
        }
        group_of_3[idx].push(line.to_string());
    }

    for g in group_of_3 {
        let mut h1 = HashSet::new();
        let mut h2 = HashSet::new();
        for ch in g[0].chars() {
            h1.insert(ch);
        }
        for ch in g[1].chars() {
            h2.insert(ch);
        }
        for ch in g[2].chars() {
            if h1.contains(&ch) && h2.contains(&ch) {
                tot += map[&ch];
                break;
            }
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

        assert_eq!(result, 157);
    }

    #[test]
    fn test_input() {
        let data = include_str!("../input.txt");

        let result = part1(data);

        assert_eq!(result, 7428);
    }

    #[test]
    fn test_example_2() {
        let data = include_str!("../example.txt");

        let result = part2(data);

        assert_eq!(result, 70);
    }

    #[test]
    fn test_input_2() {
        let data = include_str!("../input.txt");

        let result = part2(data);

        assert_eq!(result, 2650);
    }
}
