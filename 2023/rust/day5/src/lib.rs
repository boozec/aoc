use std::str::Split;

fn get_data(data: &mut Split<&str>) -> Vec<Vec<u64>> {
    data.nth(0)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim_start()
        .split('\n')
        .map(|x| x.split(' ').map(|y| y.parse::<u64>().unwrap()).collect())
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let mut input = input.trim().split("\n\n");

    let mut seeds: Vec<u64> = input
        .nth(0)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut data: Vec<Vec<Vec<u64>>> = vec![];
    for _ in 0..7 {
        let x: Vec<Vec<u64>> = get_data(&mut input);

        data.push(x);
    }
    // 0 -> seed-to-soil
    // 1 -> soil-to-fertilizer
    // 2 -> fertilizer-to-water
    // 3 -> water-to-light
    // 4 -> light-to-temperature
    // 5 -> temperature-to-humidity
    // 6 -> humidity-to-location

    for d in &data {
        let mut checked: Vec<u64> = vec![];
        for i in d {
            let drs = i[0];
            let srs = i[1];
            let incr = i[2];

            for seed in &mut seeds {
                if !checked.contains(&*seed) && *seed >= srs && *seed <= srs + incr {
                    *seed = *seed - srs + drs;
                    checked.push(*seed);
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

pub fn part2(input: &str) -> u64 {
    let mut input = input.trim().split("\n\n");

    let s: Vec<u64> = input
        .nth(0)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    assert_eq!(s.len() % 2, 0);
    let mut seeds: Vec<u64> = vec![];
    let mut i = 0;
    while i < s.len() {
        for j in s[i] as usize..(s[i] + s[i + 1]) as usize {
            seeds.push(j as u64);
        }

        i += 2;
    }
    println!("{seeds:?}");

    let mut data: Vec<Vec<Vec<u64>>> = vec![];
    for _ in 0..7 {
        let x: Vec<Vec<u64>> = get_data(&mut input);

        data.push(x);
    }
    // 0 -> seed-to-soil
    // 1 -> soil-to-fertilizer
    // 2 -> fertilizer-to-water
    // 3 -> water-to-light
    // 4 -> light-to-temperature
    // 5 -> temperature-to-humidity
    // 6 -> humidity-to-location

    for d in &data {
        let mut checked: Vec<u64> = vec![];
        for i in d {
            let drs = i[0];
            let srs = i[1];
            let incr = i[2];

            for seed in &mut seeds {
                if !checked.contains(&*seed) && *seed >= srs && *seed <= srs + incr {
                    *seed = *seed - srs + drs;
                    checked.push(*seed);
                }
            }
        }
    }

    let mut res = seeds[0];
    for seed in seeds {
        if seed < res {
            res = seed;
        }
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
        assert_eq!(result, 35);
    }

    #[test]
    fn input_part1() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 251346198);
    }

    // #[test]
    // fn example_part2() {
    //     let input = include_str!("../example.txt");
    //     let result = part2(input);
    //     assert_eq!(result, 46);
    // }
    //
    // #[test]
    // fn input_part2() {
    //     let input = include_str!("../input.txt");
    //     let result = part2(input);
    //     assert_eq!(result, 71585);
    // }
}
