use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let mut inputs: Vec<_> = buffer
        .trim()
        .split(',')
        .map(|x| x.parse::<i16>().unwrap())
        .collect::<Vec<i16>>();

    inputs.sort();

    let half: usize = inputs.len() / 2;
    let middle = inputs[half];
    let mut sum: u32 = 0;

    for input in inputs {
        sum += (input - middle).abs() as u32;
    }

    println!("{}", sum);

    Ok(())
}
