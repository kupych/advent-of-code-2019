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

fn run_intcode(input: &str, noun: i32, verb: i32) -> i32 {

    let mut numbers: Vec<i32> = input.split(",").map(|x| x.trim().parse().unwrap()).collect();
    numbers[1] = noun;
    numbers[2] = verb;

    let mut index = 0usize;

    loop {
        if index >= numbers.len() - 1 {
            break; 
        }

        let command = numbers[index];

        if command == 99 {
            break;
        }

        let pos_1 = numbers[index + 1] as usize;
        let pos_2 = numbers[index + 2] as usize;
        let target = numbers[index + 3] as usize;

        match command {
            1 => numbers[target] = numbers[pos_1] + numbers[pos_2],
            2 => numbers[target] = numbers[pos_1] * numbers[pos_2],
            _ => (),
        }

        index += 4;
    }

    return numbers[0];
}
fn part1(input: &str) -> Result<(), ()> {

    println!("Result: {}", run_intcode(input, 12, 2));
    Ok(())
}

fn part2(input: &str) -> Result<(), ()> {
    for noun in 0..99 {
        for verb in 0..99 {
            let result = run_intcode(input, noun, verb);
            if result == 19690720 {
                println!("Result: {}", noun * 100 + verb);
                return Ok(())
            }
        }
    }
    Ok(())
}
