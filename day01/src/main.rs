use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    part_1();
}

fn part_1() {
    let lines = lines_from_file("input.txt");
    let mut sum: u32 = 0;

    for line in lines {
        let mut first_value = 0;
        let mut second_value = 0;

        for char in line.chars() {
            if char.is_numeric() {
                first_value += char.to_digit(10).unwrap();
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_numeric() {
                second_value += char.to_digit(10).unwrap();
                break;
            }
        }

        sum += first_value * 10 + second_value;
    }

    println!("{}", sum);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
