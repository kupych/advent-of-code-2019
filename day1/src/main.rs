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
    let mut total = 0;

    for line in input.lines() {
        let value: i32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        total += value / 3 - 2;
    }
    println!("Result: {}", total);
    Ok(())
}

fn part2(input: &str) -> Result<(), ()> {
    let mut total = 0;

    for line in input.lines() {
        let value: i32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        total += calculate_fuel_weight(value, 0);
    }

    println!("Result: {}", total);
    Ok(())
}

fn calculate_fuel_weight(input: i32, total: i32) -> i32 {
    let fuel = input / 3 - 2;
    if fuel <= 0 {
        return total;
    }
    return calculate_fuel_weight(fuel, fuel + total);
}
