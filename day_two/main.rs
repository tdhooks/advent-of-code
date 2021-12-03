use std::env;
use std::fs;

// Enums can hold values
enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse_direction(line: &str) -> Direction {
    // Parses a line into a Direction enum
    // member with associated magnitude
    let seperated: Vec<&str> = line.split(' ').collect();
    let command = seperated[0];
    let magnitude = seperated[1].parse().unwrap();

    match command {
        "forward" => Direction::Forward(magnitude),
        "up" => Direction::Up(magnitude),
        "down" => Direction::Down(magnitude),
        _ => Direction::Forward(0),
    }
}

fn get_final_position(commands: &Vec<Direction>) -> (i32, i32) {
    // Reads through command vector and returns the final
    // position as (lateral, depth)
    let mut lateral = 0;
    let mut depth = 0;

    for command in commands {
        match command {
            Direction::Forward(mag) => lateral += mag,
            Direction::Up(mag) => depth -= mag,
            Direction::Down(mag) => depth += mag,
            _ => (),
        }
    }

    (lateral, depth)
}

fn get_final_position_aim(commands: &Vec<Direction>) -> (i32, i32) {
    // Reads through command vector, account for aim, and returns
    // the final position as (lateral, depth)
    let mut lateral = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Direction::Forward(mag) => {
                lateral += mag;
                depth += mag * aim;
            }
            Direction::Up(mag) => aim -= mag,
            Direction::Down(mag) => aim += mag,
            _ => (),
        }
    }

    (lateral, depth)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = match args.get(1) {
        Some(argument) => argument,
        None => panic!("No filename was provided."),
    };

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => panic!("Error reading file: {:?}", error),
    };

    // gets vector of Direction enum members by mapping parse_direction to each line
    let commands: Vec<Direction> = contents.lines().map(|line| parse_direction(line)).collect();

    let (part_one_lat, part_one_depth) = get_final_position(&commands);
    let part_one_result = part_one_lat * part_one_depth;

    let (part_two_lat, part_two_depth) = get_final_position_aim(&commands);
    let part_two_result = part_two_lat * part_two_depth;

    println!(
        "Part One: {}\nPart Two: {}",
        part_one_result, part_two_result
    );
}
