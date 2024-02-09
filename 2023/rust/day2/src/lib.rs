pub fn part1(input: &str) -> u32 {
    let mut res: u32 = 0;

    for (i, line) in input.trim_end().split('\n').enumerate() {
        let game = line.split(": ").collect::<Vec<&str>>()[1];
        let mut flag = true;
        for point in game.split(";") {
            for _play in point.split(',') {
                let play = _play.trim();
                let play_s: Vec<_> = play.split(' ').collect();
                let (p, c) = (play_s[0].parse::<i32>().unwrap(), play_s[1].to_string());
                if c.chars().nth(0).unwrap() == 'r' && p > 12 {
                    flag = false;
                } else if c.chars().nth(0).unwrap() == 'g' && p > 13 {
                    flag = false;
                } else if c.chars().nth(0).unwrap() == 'b' && p > 14 {
                    flag = false;
                }
            }
        }

        if flag {
            res += (i + 1) as u32;
        }
    }

    res
}

pub fn part2(input: &str) -> u32 {
    let mut res: u32 = 0;

    for line in input.trim_end().split('\n') {
        let game = line.split(": ").collect::<Vec<&str>>()[1];
        // r, g, b
        let mut balls: [u32; 3] = [0, 0, 0];
        for point in game.split(";") {
            for _play in point.split(',') {
                let play = _play.trim();
                let play_s: Vec<_> = play.split(' ').collect();
                let (p, c) = (play_s[0].parse::<u32>().unwrap(), play_s[1].to_string());

                let bi = match c.chars().nth(0).unwrap() {
                    'r' => 0,
                    'g' => 1,
                    'b' => 2,
                    _ => panic!("wtf is this color?!"),
                };

                balls[bi] = if balls[bi] < p { p } else { balls[bi] };
            }
        }

        println!("{:?}", balls);
        let mut tot = 1;

        for j in balls {
            if j != 0 {
                tot *= j;
            }
        }

        res += tot;
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
        assert_eq!(result, 8);
    }

    #[test]
    fn input_part1() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 2331);
    }

    #[test]
    fn example_part2() {
        let input = include_str!("../example.txt");
        let result = part2(input);
        assert_eq!(result, 2286);
    }

    #[test]
    fn input_part2() {
        let input = include_str!("../input.txt");
        let result = part2(input);
        assert_eq!(result, 71585);
    }
}
