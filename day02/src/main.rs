use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    part_1(12, 13, 14);
    part_2();
}

fn part_1(red: u32, green: u32, blue: u32) {
    let lines = lines_from_file("input.txt");
    let mut sum: usize = 0;

    for (i, line) in lines.iter().enumerate() {
        let game: Vec<&str> = line.split(": ").collect();
        let rounds: Vec<&str> = game[1].split("; ").collect();
        let mut game_passed = true;

        for round in rounds {
            let sets: Vec<&str> = round.split(", ").collect();
            for set in sets {
                let color: Vec<&str> = set.split(" ").collect();
                match color[1] {
                    "red" => {
                        if color[0].parse::<u32>().unwrap() > red {
                            game_passed = false
                        }
                    }
                    "green" => {
                        if color[0].parse::<u32>().unwrap() > green {
                            game_passed = false
                        }
                    }
                    "blue" => {
                        if color[0].parse::<u32>().unwrap() > blue {
                            game_passed = false
                        }
                    }
                    _ => panic!(),
                }
            }
        }

        if game_passed {
            sum += i + 1;
        }
    }
    println!("{}", sum);
}

fn part_2() {
    let lines = lines_from_file("input.txt");
    let mut sum: u32 = 0;

    for line in lines {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        let game: Vec<&str> = line.split(": ").collect();
        let rounds: Vec<&str> = game[1].split("; ").collect();

        for round in rounds {
            let sets: Vec<&str> = round.split(", ").collect();
            for set in sets {
                let color: Vec<&str> = set.split(" ").collect();
                match color[1] {
                    "red" => {
                        if color[0].parse::<u32>().unwrap() > red {
                            red = color[0].parse::<u32>().unwrap()
                        }
                    }
                    "green" => {
                        if color[0].parse::<u32>().unwrap() > green {
                            green = color[0].parse::<u32>().unwrap()
                        }
                    }
                    "blue" => {
                        if color[0].parse::<u32>().unwrap() > blue {
                            blue = color[0].parse::<u32>().unwrap()
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        sum += red * green * blue;
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
