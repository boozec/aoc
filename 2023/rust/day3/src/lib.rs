#[derive(Debug)]
struct Digit {
    value: u32,
    s: usize, // start index
    f: usize, // start end
}

#[derive(Debug)]
struct Symbol {
    elem: char,
    i: usize,
}

fn parse(s: &String, border: &i32) -> (Vec<Digit>, Vec<Symbol>) {
    let n: i32 = s.len() as i32;
    let chars = s.chars().rev();

    let mut pos1: i32 = -1;
    let mut pos2: i32 = -1;
    let mut incr = 1;
    let mut digit = 0;
    let mut symbols: Vec<Symbol> = vec![];
    let mut digits: Vec<Digit> = vec![];
    let mut multiples: Vec<usize> = vec![];

    for i in 0..150 {
        multiples.push(*border as usize * i);
    }

    for (idx, ch) in chars.enumerate() {
        if ch.is_digit(10) {
            if pos2 == -1 {
                pos2 = n - idx as i32;
            } else if multiples.contains(&idx) && pos2 != -1 {
                pos1 = n - idx as i32;
                digits.push(Digit {
                    value: digit,
                    s: pos1 as usize,
                    f: pos2 as usize,
                });
                incr = 1;
                digit = 0;
                pos2 = n - idx as i32;
            }
            digit += ch.to_digit(10).unwrap() * incr;
            incr *= 10;
        }

        if !ch.is_digit(10) {
            if digit != 0 {
                pos1 = n - idx as i32;
                if pos1 == -1 || pos2 == -1 {
                    panic!("wtf is that?!");
                }
                digits.push(Digit {
                    value: digit,
                    s: pos1 as usize,
                    f: pos2 as usize,
                })
            }
            pos2 = -1;
            incr = 1;
            digit = 0;

            if ch != '.' {
                symbols.push(Symbol {
                    elem: ch,
                    i: n as usize - idx,
                })
            }
        }
    }
    if digit != 0 {
        pos1 = 0;
        if pos2 == -1 {
            panic!("wtf is that?!");
        }
        digits.push(Digit {
            value: digit,
            s: pos1 as usize,
            f: pos2 as usize,
        })
    }

    digits.reverse();
    symbols.reverse();

    (digits, symbols)
}

pub fn part1(input: &str) -> u32 {
    let mut res: u32 = 0;

    let arr: String = input.split('\n').map(|c| c.chars()).flatten().collect();
    let border = input.find('\n').unwrap() as i32;

    let (digits, symbols) = parse(&arr, &border);

    let mut valid_s: Vec<usize> = Vec::new();
    for symbol in &symbols {
        valid_s.push(symbol.i - 1);
    }

    for digit in digits {
        // 13 with border=10
        // --> 12, 14, 2, 3, 4, 22, 23, 24
        let mut discover: Vec<usize> = vec![];
        for i in digit.s..digit.f {
            let mut js = vec![
                i as i32 - 1,
                i as i32 + 1,
                i as i32 - border - 1,
                i as i32 - border - 0,
                i as i32 - border + 1,
                // i as i32 + border - 1,
                i as i32 + border - 0,
                i as i32 + border + 1,
            ];

            if digit.f as i32 + 1 >= border {
                js.push(i as i32 + border - 1);
            }

            for j in js {
                if j >= 0
                    && !((digit.s..digit.f).contains(&(j as usize)))
                    && !(discover.contains(&(j as usize)))
                {
                    discover.push(j as usize);
                }
            }
        }

        for d in &discover {
            if valid_s.contains(&d) {
                res += digit.value;
                break;
            }
        }

        discover.sort();
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
        assert_eq!(result, 4361);
    }

    #[test]
    fn input_part1() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 556367);
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
