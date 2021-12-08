use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(grids: &Vec<Vec<u32>>, mut inputs: Vec<u32>) -> u32 {
    let mut values = Vec::<u32>::new();

    for _ in 0..4 {
        values.push(inputs.remove(0));
    }

    let mut winner: i8 = -1;
    for input in inputs {
        if winner >= 0 {
            break;
        }
        values.push(input);
        for i in 0..grids.len() {
            // Search by rows
            for j in [0, 5, 10, 15, 20] {
                let mut n = 0;
                for k in 0..5 {
                    let x = grids[i][j + k];
                    if values.iter().any(|&i| i == x) {
                        n += 1;
                    }
                }
                if n == 5 {
                    winner = i as i8;
                }
            }

            // Search by cols
            if winner < 0 {
                for j in 0..5 {
                    let mut n = 0;
                    for k in [0, 5, 10, 15, 20] {
                        let x = grids[i][j + k];
                        if values.iter().any(|&i| i == x) {
                            n += 1;
                        }
                    }
                    if n == 5 {
                        winner = i as i8;
                    }
                }
            }

            if winner >= 0 {
                break;
            }
        }
    }

    let mut sum = 0;
    for x in &grids[winner as usize] {
        if values.iter().any(|&i| i == *x) {
            continue;
        }

        sum += x;
    }

    sum * values.pop().unwrap()
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let inputs: Vec<_> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut grids: Vec<Vec<u32>> = vec![];
    let mut n: usize = 0;

    lines.next(); // Ignore the first empty line
    while let Some(line) = lines.next() {
        let mut line = line.unwrap();
        grids.push(Vec::with_capacity(5 * 5));

        for _ in 0..5 {
            grids[n].extend(
                line.trim()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            );
            line = match lines.next() {
                Some(x) => x.unwrap(),
                None => "".to_string(),
            };
        }
        n += 1;
    }

    println!("{}", part1(&grids, inputs.clone()));

    Ok(())
}
