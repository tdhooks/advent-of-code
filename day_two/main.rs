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

    let commands: Vec<Vec<&str>> = contents.lines().map(|x| x.split(' ').collect()).collect();

    println!("{:?}", commands);
}
