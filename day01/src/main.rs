use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    println!("#Part1");
    part_1();
    println!("#Part2");
    part_2();
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

fn part_2() {
    let lines = lines_from_file("input.txt");
    let mut sum: u32 = 0;
    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for line in lines {
        let first_value: u32;
        let second_value: u32;

        let mut s: String = String::from("");
        let mut arr_numbers: Vec<u32> = Vec::new();

        for char in line.chars() {
            if char.is_numeric() {
                arr_numbers.push(char.to_digit(10).unwrap());
            } else {
                s += &char.to_string();
                for (key, number) in numbers.clone().into_iter() {
                    if s.ends_with(key) {
                        arr_numbers.push(number);
                    }
                }
            }
        }

        first_value = arr_numbers[0];
        second_value = arr_numbers[arr_numbers.len() - 1];
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
