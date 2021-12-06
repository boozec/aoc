use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    let tokens: Vec<String> = reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let mut xpos: u32 = 0;
    let mut ypos: u32 = 0;
    let mut aim: u32 = 0;

    for line in tokens {
        let command: Vec<&str> = line.split(" ").collect();
        let (action, value) = (command[0], command[1].parse::<u32>().unwrap());

        match action {
            "forward" => {
                xpos += value;
                ypos += aim * value;
            }
            "down" => {
                // ypos += value;
                aim += value;
            }
            "up" => {
                // ypos -= value;
                aim -= value;
            }
            _ => {}
        }
    }
    println!("{}", ypos * xpos);
}
