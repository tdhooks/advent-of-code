use std::env;
use std::fs;

fn find_single_increments(numbers: &Vec<usize>) -> usize {
    numbers.windows(2).filter(|x| x[0] < x[1]).count()
}

fn find_grouped_increments(numbers: &Vec<usize>) -> usize {
    numbers
        .windows(4)
        .filter(|x| x[..3].iter().sum::<usize>() < x[1..].iter().sum::<usize>())
        .count()
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

    let numbers: Vec<usize> = contents.lines().map(|line| line.parse().unwrap()).collect();

    let part_one = find_single_increments(&numbers);

    let part_two = find_grouped_increments(&numbers);

    println!("Part one: {}\nPart two: {}", part_one, part_two);
}
