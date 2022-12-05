use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;

    let mut lanternfishes: Vec<_> = buffer
        .trim()
        .split(',')
        .map(|x| x.parse::<i16>().unwrap())
        .collect::<Vec<i16>>();

    for _ in 0..80 {
        let mut n = 0;
        for lanternfish in lanternfishes.iter_mut() {
            if *lanternfish == 0 {
                *lanternfish = 6;
                n += 1;
            } else {
                *lanternfish -= 1;
            }
        }

        for _ in 0..n {
            lanternfishes.push(8);
        }
    }

    println!("{}", lanternfishes.len());

    Ok(())
}
