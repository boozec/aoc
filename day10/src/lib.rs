use std::collections::VecDeque;
use std::str::FromStr;

pub struct SyntaxLines {
    lines: Vec<String>,
}

impl FromStr for SyntaxLines {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = input
            .trim()
            .split('\n')
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();

        Ok(SyntaxLines { lines })
    }
}

impl SyntaxLines {
    pub fn part1(&self) -> u32 {
        let mut result: u32 = 0;

        for line in &self.lines {
            let mut qopen = VecDeque::new();
            let mut panic_char = ' ';

            for character in line.chars() {
                match character {
                    c if character == '('
                        || character == '['
                        || character == '{'
                        || character == '<' =>
                    {
                        qopen.push_back(c);
                    }
                    c if character == ')'
                        || character == ']'
                        || character == '}'
                        || character == '>' =>
                    {
                        match qopen.pop_back() {
                            Some(bracket) => {
                                let mut error = false;
                                if c == ')' && bracket != '(' {
                                    error = true;
                                } else if c == ']' && bracket != '[' {
                                    error = true;
                                } else if c == '}' && bracket != '{' {
                                    error = true;
                                } else if c == '>' && bracket != '<' {
                                    error = true;
                                }

                                if error {
                                    panic_char = c;
                                }
                            }
                            None => panic!("Wtf?"),
                        };
                    }
                    _ => {}
                };

                if panic_char != ' ' {
                    if panic_char == ')' {
                        result += 3;
                    } else if panic_char == ']' {
                        result += 57;
                    } else if panic_char == '}' {
                        result += 1197;
                    } else if panic_char == '>' {
                        result += 25137;
                    }
                    break;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input: SyntaxLines = include_str!("../example.txt").parse().unwrap();
        let result = input.part1();
        assert_eq!(result, 26397);
    }

    #[test]
    fn test_puzzle_input() {
        let input: SyntaxLines = include_str!("../input.txt").parse().unwrap();
        let result = input.part1();
        assert_eq!(result, 392367);
    }
}
