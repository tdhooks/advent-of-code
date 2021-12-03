use std::env;
use std::fs;


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

    let numbers: Vec<Vec<u32>> = contents.lines().map(|line| line.chars().map(|c| c.to_digit(2).unwrap()).collect()).collect();

    println!("{:?}", numbers);
}
