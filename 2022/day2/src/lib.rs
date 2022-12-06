enum Player {
    Rock,
    Paper,
    Scissor,
}

impl Player {
    fn from_str(c: char) -> Self {
        match c {
            'A' | 'X' => Player::Rock,
            'B' | 'Y' => Player::Paper,
            'C' | 'Z' => Player::Scissor,
            _ => panic!("Not valid player!"),
        }
    }
}

struct Play {
    l: Player,
    r: Player,
}

impl Play {
    fn new(input: &str) -> Self {
        assert_eq!(input.len(), 3);

        Self {
            l: Player::from_str(input.chars().nth(0).unwrap()),
            r: Player::from_str(input.chars().nth(2).unwrap()),
        }
    }

    fn points(&self) -> u32 {
        let mut tot: u32 = 0;

        match self.r {
            Player::Rock => {
                tot += 1;

                tot += match self.l {
                    Player::Rock => 3,
                    Player::Scissor => 6,
                    _ => 0,
                };
            }
            Player::Paper => {
                tot += 2;

                tot += match self.l {
                    Player::Paper => 3,
                    Player::Rock => 6,
                    _ => 0,
                };
            }
            Player::Scissor => {
                tot += 3;

                tot += match self.l {
                    Player::Scissor => 3,
                    Player::Paper => 6,
                    _ => 0,
                };
            }
        }

        tot
    }

    fn calc_secret_points(&mut self) {
        match self.r {
            // Means "Lose"
            Player::Rock => {
                match self.l {
                    Player::Rock => {
                        self.r = Player::Scissor;
                    }
                    Player::Scissor => {
                        self.r = Player::Paper;
                    }
                    _ => {}
                };
            }
            // Means "Draw"
            Player::Paper => {
                match self.l {
                    Player::Scissor => {
                        self.r = Player::Scissor;
                    }
                    Player::Rock => {
                        self.r = Player::Rock;
                    }
                    _ => {}
                };
            }
            // Means "Win"
            Player::Scissor => {
                match self.l {
                    Player::Scissor => {
                        self.r = Player::Rock;
                    }
                    Player::Rock => {
                        self.r = Player::Paper;
                    }
                    _ => {}
                };
            }
        };
    }
}

fn get_plays(input: &str) -> Vec<Play> {
    input
        .trim()
        .split('\n')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| Play::new(x))
        .collect::<Vec<Play>>()
}

pub fn part1(input: &str) -> u32 {
    let mut tot: u32 = 0;
    let rows = get_plays(input);

    for row in rows {
        tot += row.points();
    }

    tot
}

pub fn part2(input: &str) -> u32 {
    let mut tot: u32 = 0;

    let mut rows = get_plays(input);

    for row in &mut rows {
        row.calc_secret_points();
        tot += row.points();
    }

    tot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let data = include_str!("../example.txt");

        let result = part1(data);

        assert_eq!(result, 15);
    }

    #[test]
    fn test_input() {
        let data = include_str!("../input.txt");

        let result = part1(data);

        assert_eq!(result, 11906);
    }

    #[test]
    fn test_example_part2() {
        let data = include_str!("../example.txt");

        let result = part2(data);

        assert_eq!(result, 12);
    }

    #[test]
    fn test_input_part2() {
        let data = include_str!("../input.txt");

        let result = part2(data);

        assert_eq!(result, 11186);
    }
}
