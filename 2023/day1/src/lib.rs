pub fn part1(input: &str) -> u32 {
    let mut res: u32 = 0;

    for line in input.split('\n') {
        let mut d1: i32 = -1;
        let mut d2: i32 = -1;
        for ch in line.chars() {
            if ch.is_digit(10) {
                if d1 == -1 {
                    d1 = ch.to_digit(10).unwrap() as i32 * 10;
                }

                d2 = ch.to_digit(10).unwrap() as i32;
            }
        }

        if d1 + d2 > 0 {
            res += (d1 + d2) as u32;
        }
    }

    res
}

pub fn part2(input: &str) -> u32 {
    let mut res: u32 = 0;

    let hs: [(&str, u32); 18] = [
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ];

    for (i, line) in input.strip_suffix('\n').unwrap().split('\n').enumerate() {
        let mut idx: Vec<(i32, u32)> = vec![];
        for (k, v) in &hs {
            let ms: Vec<_> = line.match_indices(k).collect();
            for m in ms.iter() {
                idx.push((m.0 as i32, *v));
            }
        }

        idx.sort_by(|a, b| a.0.cmp(&b.0));
        println!("{i} {}", idx[0].1 * 10 + idx[idx.len() - 1].1);
        res += idx[0].1 * 10 + idx[idx.len() - 1].1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = include_str!("../example.txt");
        let result = part1(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn input_part1() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 54159);
    }

    #[test]
    fn example_part2() {
        let input = include_str!("../example2.txt");
        let result = part2(input);
        assert_eq!(result, 281);
    }

    #[test]
    fn input_part2() {
        let input = include_str!("../input2.txt");
        let result = part2(input);
        assert_eq!(result, 53866);
    }
}
