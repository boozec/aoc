pub fn part1(input: &str) -> i32 {
    let mut res: i32 = 0;

    let m: Vec<Vec<i32>> = input
        .trim_end()
        .split('\n')
        .map(|x| {
            x.split(' ')
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    for i in m {
        let mut aux: Vec<Vec<i32>> = vec![i];

        let mut z = 1;
        loop {
            aux.push(vec![]);
            for j in 1..aux[z - 1].len() {
                let x = aux[z - 1][j] - aux[z - 1][j - 1];
                aux[z].push(x);
            }

            if aux[z].iter().filter(|&k| *k != 0).count() == 0 {
                aux[z].push(0);
                break;
            }

            z += 1;
        }

        let n = aux.len() - 1;
        for j in (0..n).rev() {
            let t = aux[j + 1][aux[j + 1].len() - 1];
            let r = aux[j][aux[j].len() - 1];
            aux[j].push(t + r);
        }

        res += aux[0][aux[0].len() - 1];
    }

    res
}

pub fn part2(input: &str) -> i32 {
    let mut res: i32 = 0;

    let m: Vec<Vec<i32>> = input
        .trim_end()
        .split('\n')
        .map(|x| {
            x.split(' ')
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    for i in m {
        let mut aux: Vec<Vec<i32>> = vec![i];

        let mut z = 1;
        loop {
            aux.push(vec![]);
            for j in 1..aux[z - 1].len() {
                let x = aux[z - 1][j] - aux[z - 1][j - 1];
                aux[z].push(x);
            }

            if aux[z].iter().filter(|&k| *k != 0).count() == 0 {
                aux[z].push(0);
                break;
            }

            z += 1;
        }

        let n = aux.len() - 1;
        for j in (0..n).rev() {
            let t = aux[j + 1][0];
            let r = aux[j][0];
            aux[j].insert(0, r - t);
        }

        res += aux[0][0];
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
        assert_eq!(result, 114);
    }

    #[test]
    fn input_part1() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 2008960228);
    }

    #[test]
    fn example_part2() {
        let input = include_str!("../example.txt");
        let result = part2(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn input_part2() {
        let input = include_str!("../input.txt");
        let result = part2(input);
        assert_eq!(result, 71585);
    }
}
