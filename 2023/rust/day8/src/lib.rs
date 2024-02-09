use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let mut res: u32 = 0;

    let lines: Vec<String> = input
        .trim_end()
        .split('\n')
        .map(|x| x.to_string())
        .collect();
    let ltype = &lines[0];
    let mut hm: HashMap<String, (String, String)> = HashMap::new();
    for play in &lines[2..] {
        hm.insert(
            play[..3].to_string(),
            (play[7..10].to_string(), play[12..15].to_string()),
        );
    }

    let mut current = "AAA".to_string();
    for i in ltype.chars().cycle() {
        current = match i {
            'R' => hm.get(&current).unwrap().1.clone(),
            'L' => hm.get(&current).unwrap().0.clone(),
            _ => panic!("wtf is this?"),
        };
        res += 1;

        if current == "ZZZ".to_string() {
            break;
        }
    }

    res
}

pub fn part2(input: &str) -> u32 {
    let mut res: u32 = 0;

    let lines: Vec<String> = input
        .trim_end()
        .split('\n')
        .map(|x| x.to_string())
        .collect();
    let ltype = &lines[0];
    let mut hm: HashMap<String, (String, String)> = HashMap::new();

    for play in &lines[2..] {
        hm.insert(
            play[..3].to_string(),
            (play[7..10].to_string(), play[12..15].to_string()),
        );
    }

    let mut currents: Vec<String> = hm
        .keys()
        .filter(|x| x.ends_with("A"))
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    dbg!("{currents:?}");
    'o: for i in ltype.chars().cycle() {
        for k in 0..currents.len() {
            currents[k] = match i {
                'R' => hm.get(&currents[k]).unwrap().1.clone(),
                'L' => hm.get(&currents[k]).unwrap().0.clone(),
                _ => panic!("wtf is this?"),
            };
            res += 1;
        }

        for current in &currents {
            if !current.ends_with("Z") {
                continue 'o;
            }
        }
        break;
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
        assert_eq!(result, 2);
    }

    #[test]
    fn input_part1() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 21389);
    }

    #[test]
    fn example_part2() {
        let input = include_str!("../example2.txt");
        let result = part2(input);
        assert_eq!(result, 12);
    }

    // #[test]
    // fn input_part2() {
    //     let input = include_str!("../input.txt");
    //     let result = part2(input);
    //     assert_eq!(result, 248179786);
    // }
}
