pub fn get_sums(input: &str) -> Vec<i32> {
    let groups: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();
    let data_groups = groups
        .into_iter()
        .map(|x| {
            let numbers: i32 = x
                .split('\n')
                .map(|x| x.parse::<i32>().unwrap_or_default())
                .sum();

            return numbers;
        })
        .collect::<Vec<i32>>();

    data_groups
}

pub fn part1(input: &str) -> i32 {
    let sums = get_sums(input);

    sums.into_iter().max().unwrap()
}

pub fn part2(input: &str) -> i32 {
    let mut sums = get_sums(input);

    sums.sort();
    sums.reverse();

    sums[0] + sums[1] + sums[2]
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn test_example() {
        let data = include_str!("../example.txt");

        let result = part1(data);

        assert_eq!(result, 24000);
    }

    #[test]
    fn test_input() {
        let data = include_str!("../input.txt");

        let result = part1(data);

        assert_eq!(result, 69912);
    }

    #[test]
    fn test_example_part2() {
        let data = include_str!("../example.txt");

        let result = part2(data);

        assert_eq!(result, 45000);
    }

    #[test]
    fn test_input_part2() {
        let data = include_str!("../input.txt");

        let result = part2(data);

        assert_eq!(result, 208180);
    }
}
