use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let path = "./src/binary_diagnostic/input";
    let data = parse_input(path);

    // Calculations
    let gamma = calculate_gamma(&data);
    let epsilon = calculate_epsilon(gamma);

    println!("{} * {} = {}",gamma, epsilon, epsilon*gamma);

    let oxygen_data = data.clone();
    let oxygen = calculate_oxygen_generator(oxygen_data, 11);
    let co2_data = data.clone();
    let co2 = calculate_co2_scrubber(co2_data, 11);

    println!("{} * {} = {}", oxygen, co2, oxygen*co2);
}

fn parse_input(path: &str) -> Vec<usize> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    // Read binary data into vector
    for line in reader.lines() {
        let line = line.unwrap();

        let mut bit_place = 11;
        let mut total = 0;

        for c in line.chars() {
            match c {
                '0' => bit_place-=1,
                '1' => {
                    total += 2_usize.pow(bit_place as u32);
                    bit_place-=1;
                }
                _ => {}
            }
        }

        data.push(total);
    }

    data
}

fn calculate_gamma(data: &Vec<usize>) -> usize {
    let mut gamma = 0;
    let count = data.len();

    for i in 0..12 {
        let bit_count = bit_count(data, i);
        if bit_count > count/2 {
            gamma |= 1 << i;
        }
    }

    gamma
}

fn calculate_epsilon(gamma: usize) -> usize {
    gamma ^ 0b11111
}

fn calculate_oxygen_generator(candidates: Vec<usize>, place: usize) -> usize {
    let count = candidates.len();

    let bit_count = bit_count(&candidates, place);
    let more1 = bit_count >= count-bit_count;
    let mask = 0 | (1 << place);

    let candidates = candidates.iter().filter(|e| {
        let val = (*e&mask) >> place;
        return (val==1 && more1) || (val==0 && !more1)
    }).map(|e| *e)
    .collect::<Vec<usize>>();

    if candidates.len() == 1 {
        return candidates[0];
    } else {
        return calculate_oxygen_generator(candidates, place-1);
    }
}

fn calculate_co2_scrubber(candidates: Vec<usize>, place: usize) -> usize {
    let count = candidates.len();

    let bit_count = bit_count(&candidates, place);
    let more1 = bit_count >= count-bit_count;
    let mask = 0 | (1 << place);

    let candidates = candidates.iter().filter(|e| {
        let val = (*e&mask) >> place;
        return (val==1 && !more1) || (val==0 && more1);
    }).map(|e| *e)
    .collect::<Vec<usize>>();

    if candidates.len() == 1 {
        return candidates[0];
    }

    calculate_co2_scrubber(candidates, place-1)
}

fn bit_count(data: &Vec<usize>, place: usize) -> usize {
    let mask = 0 | (1 << place);
    let mut bit_count = 0;

    for datum in data {
        if ((datum&mask) >> place) == 1 {
            bit_count += 1;
        }
    }

    bit_count
}
