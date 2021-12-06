use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(file);
    let values: Vec<i32> = reader
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut increasing: u16 = 0;

    for (index, _) in values.iter().enumerate() {
        if index == 0 {
            continue;
        }

        if values[index - 1] < values[index] {
            increasing += 1;
        }
    }

    println!("{}", increasing);
}
