use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let path = "./src/treachery_of_whales/input";
    let crabs = parse_input(path);

    let mut champ = None;
    let mut champ_pos = None;
    for pos in 0..*crabs.iter().max().unwrap() {
        let fuel = calculate_fuel(&crabs, pos);
        if champ.is_none() || champ.unwrap() > fuel {
            champ = Some(fuel);
            champ_pos = Some(pos)
        }
    }

    println!("{}", champ.unwrap());
    println!("{}", champ_pos.unwrap());
}

fn parse_input(path: &str) -> Vec<usize> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let line = reader.lines().next().unwrap().unwrap();
    line.split(",")
        .map(|e| {
            e.parse::<usize>().unwrap()
        })
        .collect()
}

fn calculate_fuel(crabs: &Vec<usize>, pos: usize) -> usize {
    let mut total = 0;
    for crab in crabs {
        let delta = (*crab as isize - pos as isize).abs() as usize;

        total += (delta*(delta+1))/2;
    }

    total
}
