fn parse(input: &str) -> Vec<String> {
    input
        .trim_end()
        .split('\n')
        .map(|x| {
            let b = x.to_string();
            let k = b.split(':').last().unwrap();

            return k.trim().to_string();
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let mut res: u32 = 0;

    let cards: Vec<String> = parse(input);
    for card in cards {
        let numbers: Vec<String> = card.split('|').map(|x| x.trim().to_string()).collect();
        assert_eq!(numbers.len(), 2);

        let nums: Vec<Vec<_>> = numbers
            .iter()
            .map(|x| {
                x.replace("  ", " ")
                    .split(' ')
                    .map(|x| x.trim().parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        assert_eq!(nums.len(), 2);
        let (winnings, plays) = (&nums[0], &nums[1]);

        let mut k = 0.5;
        for win in winnings {
            if plays.contains(&win) {
                k *= 2.;
            }
        }
        res += k as u32;
    }

    res
}

//
//
//
// 1 -> 4 | x
// 2 -> 2 | y x
// 3 -> 2 | y y y x
// 4 -> 1 | y y y y y y y x
// 5 -> 0 | y y y y y y y y y y y y y x
// 6 -> 0 | x
//
//
pub fn part2(input: &str) -> u32 {
    let mut res: u32 = 0;

    let cards: Vec<String> = parse(input);
    let mut incr: Vec<u32> = Vec::new();
    incr.resize_with(cards.len(), Default::default);

    let mut i = 0;
    for card in cards {
        let numbers: Vec<String> = card.split('|').map(|x| x.trim().to_string()).collect();
        assert_eq!(numbers.len(), 2);

        let nums: Vec<Vec<_>> = numbers
            .iter()
            .map(|x| {
                x.replace("  ", " ")
                    .split(' ')
                    .map(|x| x.trim().parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        assert_eq!(nums.len(), 2);
        let (winnings, plays) = (&nums[0], &nums[1]);

        let mut k = 0;
        for win in winnings {
            if plays.contains(&win) {
                k += 1;
            }
        }

        for j in (i + 1)..(i + 1 + k) {
            incr[j] += 1 + incr[i];
        }

        res += 1 + incr[i];
        i += 1;
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
        assert_eq!(result, 13);
    }

    #[test]
    fn input_part1() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 25174);
    }

    #[test]
    fn example_part2() {
        let input = include_str!("../example.txt");
        let result = part2(input);
        assert_eq!(result, 30);
    }

    #[test]
    fn input_part2() {
        let input = include_str!("../input.txt");
        let result = part2(input);
        assert_eq!(result, 6420979);
    }
}
