use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let path = "./src/lanternfish/input";
    let fish = parse_input(path);

    let mut fish_freq = [0; 9];

    for f in fish {
        fish_freq[f as usize] += 1;
    }

    for _ in 0..256 {
        let birth_count = fish_freq[0];

        for f in 1..9 {
            fish_freq[f-1] = fish_freq[f];
        }

        fish_freq[8] = birth_count;
        fish_freq[6] += birth_count;
    }

    let count: u64 = fish_freq.iter().map(|e| e).sum();

    println!("{} fish after 256 days", count);
}

fn parse_input(path: &str) -> Vec<u8> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut fish = Vec::new();
    let input = reader.lines().next().unwrap().unwrap();

    for f in input.split(",") {
        fish.push(f.parse::<u8>().unwrap());
    }

    fish
}
