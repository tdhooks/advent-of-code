use std::env;
use std::fs;

fn get_decimal_value(bits: &Vec<u32>) -> u32 {
    bits.iter().fold(0, |acc, bit| (acc << 1) | bit)
}

fn idiomatic_negate_bit(bit: u32) -> u32 {
    match bit {
        0 => 1,
        1 => 0,
        _ => 0,
    }
}

fn idiomatic_negate_bits(bits: &Vec<u32>) -> Vec<u32> {
    bits.iter().map(|&bit| idiomatic_negate_bit(bit)).collect()
}

fn mode_through_column(bits: &Vec<Vec<u32>>, col_idx: usize) -> Option<u32> {
    // assumes only values are 1 and 0
    let col_sum = bits.iter().fold(0, |acc, bitfield| acc + bitfield[col_idx]);

    match col_sum {
        s if s < bits.len() as u32 - s => Some(0),
        s if s > bits.len() as u32 - s => Some(1),
        _ => None,
    }
}

fn get_gamma_epsilon(bits: &Vec<Vec<u32>>) -> (u32, u32) {
    let gamma_rate_bits = (0..bits[0].len())
        .map(|idx| mode_through_column(bits, idx).unwrap())
        .collect();

    let epsilon_rate_bits = idiomatic_negate_bits(&gamma_rate_bits);

    (
        get_decimal_value(&gamma_rate_bits),
        get_decimal_value(&epsilon_rate_bits),
    )
}

fn filter_oxygen_rating(bits: &Vec<Vec<u32>>, col_idx: usize) -> Vec<Vec<u32>> {
    let col_mode = mode_through_column(bits, col_idx);

    if bits.len() == 1 || col_idx == bits[0].len() {
        return bits.clone();
    }
    // not cleaner to directly pass col_mode because need to interpret edge case output
    match col_mode {
        Some(0) => filter_oxygen_rating(
            &bits
                .clone()
                .into_iter()
                .filter(|bitfield| bitfield[col_idx] == 0)
                .collect(),
            col_idx + 1,
        ),
        Some(1) => filter_oxygen_rating(
            &bits
                .clone()
                .into_iter()
                .filter(|bitfield| bitfield[col_idx] == 1)
                .collect(),
            col_idx + 1,
        ),
        None => filter_oxygen_rating(
            &bits
                .clone()
                .into_iter()
                .filter(|bitfield| bitfield[col_idx] == 1)
                .collect(),
            col_idx + 1,
        ),
        _ => panic!("wat"),
    }
}

fn filter_co2_rating(bits: &Vec<Vec<u32>>, col_idx: usize) -> Vec<Vec<u32>> {
    let col_mode = mode_through_column(bits, col_idx);

    if bits.len() == 1 || col_idx == bits[0].len() {
        return bits.clone();
    }
    // same as above case but also need to negate, unfortunately
    // easier to just handle negate in this match vs using helper
    match col_mode {
        Some(0) => filter_co2_rating(
            &bits
                .clone()
                .into_iter()
                .filter(|bitfield| bitfield[col_idx] == 1)
                .collect(),
            col_idx + 1,
        ),
        Some(1) => filter_co2_rating(
            &bits
                .clone()
                .into_iter()
                .filter(|bitfield| bitfield[col_idx] == 0)
                .collect(),
            col_idx + 1,
        ),
        None => filter_co2_rating(
            &bits
                .clone()
                .into_iter()
                .filter(|bitfield| bitfield[col_idx] == 0)
                .collect(),
            col_idx + 1,
        ),
        _ => panic!("wat"),
    }
}

fn get_oxygen_rating(bits: &Vec<Vec<u32>>) -> u32 {
    let wrapped_rating = filter_oxygen_rating(&bits, 0);

    get_decimal_value(&wrapped_rating[0])
}

fn get_co2_rating(bits: &Vec<Vec<u32>>) -> u32 {
    let wrapped_rating = filter_co2_rating(&bits, 0);

    get_decimal_value(&wrapped_rating[0])
}

fn get_oxygen_co2(bits: &Vec<Vec<u32>>) -> (u32, u32) {
    (get_oxygen_rating(bits), get_co2_rating(bits))
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

    let bits: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(2).unwrap()).collect())
        .collect();

    let (gamma, epsilon) = get_gamma_epsilon(&bits);
    let (oxygen, co2) = get_oxygen_co2(&bits);

    println!(
        "Day one: gamma = {}, epsilon = {}, product = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    println!(
        "Day two: oxygen = {}, co2 = {}, product = {}",
        oxygen,
        co2,
        oxygen * co2
    );
}
