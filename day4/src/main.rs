use std::env;
use std::fs;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();

    let part_arg = &args[1];

    let input = fs::read_to_string("./input/input.txt")
        .expect("Could not read input file");

    if part_arg == "1" {
        part1(&input)?;
    }

    if part_arg == "2" {
        part2(&input)?;
    }

    Ok(())
}

fn part1(input: &str) -> Result<(), ()> {
    for line in input.lines() {
    // Solution goes here 
    }
    Ok(())
}

fn part2(input: &str) -> Result<(), ()> {
    for line in input.lines() {
    // Solution goes here 
    }
    Ok(())
}
