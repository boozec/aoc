pub fn part1(input: &str) -> u32 {
    let mut res: u32 = 0;

    let inputs: Vec<String> = input.trim_end().split(',').map(|x| x.to_string()).collect();

    for i in inputs {
        let mut s = 0;
        for ch in i.chars() {
            s += ch as u32;
            s *= 17;
            s %= 256;
        }
        res += s;
    }

    res
}

pub fn part2(input: &str) -> u32 {
    let mut res: u32 = 0;

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = include_str!("../example.txt");
        let result = part1(input);
        assert_eq!(result, 1320);
    }

    #[test]
    fn input_part1() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 2331);
    }

    // #[test]
    // fn example_part2() {
    //     let input = include_str!("../example.txt");
    //     let result = part2(input);
    //     assert_eq!(result, 2286);
    // }
    //
    // #[test]
    // fn input_part2() {
    //     let input = include_str!("../input.txt");
    //     let result = part2(input);
    //     assert_eq!(result, 71585);
    // }
}
