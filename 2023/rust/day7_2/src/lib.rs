use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash)]
enum HandType {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}

#[derive(Debug)]
struct Play {
    hand: Vec<u32>,
    bid: u32,
    hand_type: HandType,
}

#[derive(Debug)]
struct PlayError;

impl FromStr for Play {
    type Err = PlayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains(' ') {
            return Err(PlayError);
        }

        let s: Vec<String> = s.split(' ').map(|x| x.to_string()).collect();
        if s.len() != 2 {
            return Err(PlayError);
        }

        let mut diffcards: HashMap<char, u32> = HashMap::new();
        let hand = s[0].chars().collect::<Vec<char>>();

        for ch in &hand {
            if diffcards.contains_key(&ch) {
                *(diffcards.get_mut(ch).unwrap()) += 1;
            } else {
                diffcards.insert(*ch, 1);
            }
        }

        let card_points: HashMap<char, u32> = HashMap::from([
            ('A', 13),
            ('K', 12),
            ('Q', 11),
            ('T', 9),
            ('9', 8),
            ('8', 7),
            ('7', 6),
            ('6', 5),
            ('5', 4),
            ('4', 3),
            ('3', 2),
            ('2', 1),
            ('J', 0),
        ]);
        let hand: Vec<_> = hand.iter().map(|x| *card_points.get(&x).unwrap()).collect();
        let jokers = hand.iter().filter(|&n| *n == 0).count();
        let b: Vec<u32> = diffcards.clone().into_values().collect();
        let max_labels: u32 = *b.iter().max().unwrap();

        let mut hand_type: HandType = match diffcards.keys().len() {
            1 => HandType::Five,
            2 if max_labels == 4 => HandType::Four,
            2 if max_labels == 3 => HandType::Full,
            3 if max_labels == 3 => HandType::Three,
            3 => HandType::Two,
            4 => HandType::One,
            5 => HandType::High,
            _ => panic!("wtf is this?"),
        };

        match jokers {
            1 => {
                hand_type = match hand_type {
                    HandType::High => HandType::One,
                    HandType::One => HandType::Three,
                    HandType::Two => HandType::Full,
                    HandType::Three => HandType::Four,
                    HandType::Four => HandType::Five,
                    _ => panic!("wtf is this?"),
                }
            }
            2 => {
                hand_type = match hand_type {
                    HandType::One => HandType::Three,
                    HandType::Two => HandType::Four,
                    HandType::Full => HandType::Five,
                    _ => panic!("wtf is this?"),
                }
            }
            3 => {
                hand_type = match hand_type {
                    HandType::High => HandType::Three,
                    HandType::One => HandType::Four,
                    HandType::Two => HandType::Five,
                    HandType::Three => HandType::Four,
                    HandType::Full => HandType::Five,
                    _ => panic!("wtf is this?"),
                }
            }
            4 => {
                hand_type = HandType::Five;
            }
            0 | 5 => {}
            _ => panic!("wtf is this?"),
        };

        Ok(Play {
            hand,
            bid: s[1].parse::<u32>().unwrap(),
            hand_type,
        })
    }
}

pub fn part2(input: &str) -> u32 {
    let mut res: u32 = 0;

    let plays: Vec<_> = input
        .trim_end()
        .split('\n')
        .map(|x| x.parse::<Play>().unwrap())
        .collect();

    let mut hs: HashMap<HandType, Vec<Play>> = HashMap::from([
        (HandType::Five, vec![]),
        (HandType::Four, vec![]),
        (HandType::Full, vec![]),
        (HandType::Three, vec![]),
        (HandType::Two, vec![]),
        (HandType::One, vec![]),
        (HandType::High, vec![]),
    ]);

    for play in plays {
        if let Some(p) = hs.get_mut(&play.hand_type) {
            p.push(play);
        }
    }

    let mut rank = 1;

    for kind in [
        HandType::High,
        HandType::One,
        HandType::Two,
        HandType::Three,
        HandType::Full,
        HandType::Four,
        HandType::Five,
    ] {
        hs.get_mut(&kind)
            .unwrap()
            .sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());

        for h in hs.get(&kind).unwrap() {
            res += rank * h.bid;
            rank += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part2() {
        let input = include_str!("../example.txt");
        let result = part2(input);
        assert_eq!(result, 5905);
    }

    #[test]
    fn input_part2() {
        let input = include_str!("../input.txt");
        let result = part2(input);
        assert_eq!(result, 247885995);
    }
}
