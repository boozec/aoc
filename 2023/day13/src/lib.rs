pub fn part1(input: &str) -> u32 {
    let mut res: u32 = 0;

    let m: Vec<Vec<String>> = input
        .trim_end()
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|y| y.chars().collect::<String>())
                .collect()
        })
        .collect();

    for s in m {
        let n = s.len();

        // Search by column
        //  012345678
        //      ><
        //  #.##..##.
        //  ..#.##.#.
        //  ##......#
        //  ##......#
        //  ..#.##.#.
        //  ..##..##.
        //  #.#.##.#.
        //      ><
        //  012345678
        //
        let m = s[0].len();
        let mut mc = 0;
        for i in 0..m {
            let mut j = m - 1;
            while j > i {
                let mut s1 = String::new();
                let mut s2 = String::new();
                for k in 0..n {
                    s1.push(s[k].chars().nth(i).unwrap());
                    s2.push(s[k].chars().nth(j).unwrap());
                }
                if s1 == s2 {
                    mc = std::cmp::max(mc, i);
                }
                j -= 1;
            }
        }

        let mut check = true;

        let mut i = mc;
        let mut j = mc + 1;

        while i > 0 && j < n {
            let mut s1 = String::new();
            let mut s2 = String::new();
            for k in 0..n {
                s1.push(s[k].chars().nth(i).unwrap());
                if j >= s[k].len() {
                    break;
                }
                s2.push(s[k].chars().nth(j).unwrap());
            }
            if s1 != s2 {
                check = false;
                break;
            }
            i -= 1;
            j += 1;
        }

        if check {
            res += (mc + 1) as u32;
            continue;
        }

        // Search by row
        let mut mc = 0;
        for i in 0..(n / 2) + 1 {
            for j in ((n / 2)..n).rev() {
                if s[i] == s[j] {
                    mc = std::cmp::max(mc, i);
                }
            }
        }

        let mut check = true;

        let mut i = mc;
        let mut j = mc + 1;

        while i > 0 && j < n {
            if s[i] != s[j] {
                check = false;
                break;
            }
            i -= 1;
            j += 1;
        }

        if check {
            res += (mc + 1) as u32 * 100;
            continue;
        }
    }

    res
}

// pub fn part2(input: &str) -> u32 {
//     let mut res: u32 = 0;
//
//     res
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = include_str!("../example.txt");
        let result = part1(input);
        assert_eq!(result, 405);
    }

    #[test]
    fn input_part1() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 2331);
    }
    //
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
