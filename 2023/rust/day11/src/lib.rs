fn expand(m: &mut Vec<Vec<i16>>, ch: i16) {
    let rows = m.len();
    let mut cols = m[0].len();

    let mut i = 0;
    while i < rows {
        if m[i].iter().filter(|&x| *x != 0).count() == 0 {
            m.insert(i, vec![ch; cols]);
            i += 1;
        }
        i += 1;
    }
    let rows = m.len();
    i = 0;

    while i < cols {
        let mut c = 0;
        for j in 0..rows {
            if m[j][i] != 0 {
                c += 1;
            }
        }

        if c == 0 {
            for j in 0..rows {
                m[j].insert(i, ch);
            }
            cols += 1;
            i += 1;
        }
        i += 1;
    }
}

pub fn part1(input: &str) -> u32 {
    let mut res: u32 = 0;

    let mut m: Vec<Vec<i16>> = input
        .trim_end()
        .split('\n')
        .map(|y| y.chars().map(|x| if x == '#' { 1 } else { 0 }).collect())
        .collect();

    expand(&mut m, 0);
    let mut hs: Vec<(usize, usize)> = vec![];
    let rows = m.len();
    let cols = m[0].len();
    for i in 0..rows {
        for j in 0..cols {
            if m[i][j] != 0 {
                hs.push((i, j));
            }
        }
    }

    let n = hs.len();
    for i in 0..n {
        for j in i + 1..n {
            res += ((hs[j].1 as i16 - hs[i].1 as i16).abs()
                + (hs[j].0 as i16 - hs[i].0 as i16).abs()) as u32;
        }
    }

    res
}

pub fn part2(input: &str) -> u32 {
    let mut res: u32 = 0;

    let mut m: Vec<Vec<i16>> = input
        .trim_end()
        .split('\n')
        .map(|y| y.chars().map(|x| if x == '#' { 1 } else { 0 }).collect())
        .collect();

    expand(&mut m, 1);
    let mut hs: Vec<(usize, usize)> = vec![];
    let rows = m.len();
    let cols = m[0].len();
    for i in 0..rows {
        for j in 0..cols {
            if m[i][j] > 0 {
                hs.push((i, j));
            }
        }
    }

    let n = hs.len();
    for i in 0..n {
        for j in i + 1..n {
            res += ((hs[j].1 as i16 - hs[i].1 as i16).abs()
                + (hs[j].0 as i16 - hs[i].0 as i16).abs()) as u32;
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
        assert_eq!(result, 374);
    }

    #[test]
    fn input_part1() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 9623138);
    }

    #[test]
    fn example_part2() {
        let input = include_str!("../example.txt");
        let result = part2(input);
        assert_eq!(result, 1030);
    }
    //
    // #[test]
    // fn input_part2() {
    //     let input = include_str!("../input.txt");
    //     let result = part2(input);
    //     assert_eq!(result, 71585);
    // }
}
