pub fn part1(input: &str) -> u64 {
    let mut res: u64 = 1;

    let td: Vec<Vec<_>> = input
        .trim_end()
        .split('\n')
        .map(|x| {
            x.split(':')
                .nth(1)
                .unwrap()
                .trim()
                .replace("    ", " ")
                .replace("   ", " ")
                .replace("  ", " ")
                .split(' ')
                .map(|y| y.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    assert_eq!(td.len(), 2);
    let (t, d) = (&td[0], &td[1]);

    for i in 0..t.len() {
        let mut r = 0;
        for j in 0..t[i] {
            if (t[i] - j) * j > d[i] {
                r += 1;
            }
        }
        res *= r;
    }

    res
}

pub fn part2(input: &str) -> u64 {
    let mut res: u64 = 1;

    let td: Vec<Vec<_>> = input
        .trim_end()
        .split('\n')
        .map(|x| {
            x.split(':')
                .nth(1)
                .unwrap()
                .trim()
                .replace(" ", "")
                .split(' ')
                .map(|y| y.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    assert_eq!(td.len(), 2);
    let (t, d) = (&td[0], &td[1]);

    for i in 0..t.len() {
        let mut r = 0;
        for j in 0..t[i] {
            if (t[i] - j) * j > d[i] {
                r += 1;
            }
        }
        res *= r;
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
        assert_eq!(result, 288);
    }

    #[test]
    fn input_part1() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 74698);
    }

    #[test]
    fn example_part2() {
        let input = include_str!("../example.txt");
        let result = part2(input);
        assert_eq!(result, 71503);
    }

    #[test]
    fn input_part2() {
        let input = include_str!("../input.txt");
        let result = part2(input);
        assert_eq!(result, 27563421);
    }
}
