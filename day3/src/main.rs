use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(&file);

    let lines: Vec<_> = reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let mut bits: Vec<u32> = vec![0; lines[0].len()];
    let half: u32 = (lines.len() / 2) as u32;
    let mut gamma_string = String::new();
    let mut epsilon_string = String::new();

    for line in lines {
        for (index, character) in line.chars().enumerate() {
            bits[index] += character.to_digit(10).unwrap();
        }
    }

    for bit in bits {
        if bit >= half {
            gamma_string.push('1');
            epsilon_string.push('0');
        } else {
            gamma_string.push('0');
            epsilon_string.push('1');
        }
    }
    let gamma_rate = isize::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon_string, 2).unwrap();

    println!("{}", gamma_rate * epsilon_rate);

    Ok(())
}
