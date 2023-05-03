pub fn part(input: &str, m: usize) -> usize {
    let n = input.len();

    for i in 0..n {
        let s = &input[i..i + m];
        let mut check = false;
        if s.trim().len() < m {
            break;
        }

        'o: for j in 0..m {
            for k in 0..m {
                if j == k {
                    continue;
                }

                if s.chars().nth(j) == s.chars().nth(k) {
                    check = true;
                    break 'o;
                }
            }
        }

        if !check {
            return i + m;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let result = part(input, 4);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let result = part(input, 4);

        assert_eq!(result, 1779);
    }

    #[test]
    fn test_example2() {
        let input = include_str!("../example.txt");
        let result = part(input, 14);

        assert_eq!(result, 23);
    }

    #[test]
    fn test_input2() {
        let input = include_str!("../input.txt");
        let result = part(input, 14);

        assert_eq!(result, 23);
    }
}
