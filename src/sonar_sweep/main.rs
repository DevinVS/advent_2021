use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let mut sonar_values = Vec::new();

    // Read and parse the inputs from the file
    let path = "./src/sonar_sweep/input";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            sonar_values.push(line.trim().parse::<u32>().unwrap());
        }
    }

    // Iterator over the sonar values
    let mut count = 0;

    // Calculate the number of variables that increased and decresed
    for i in 0..sonar_values.len() {
        let x = sonar_values[i];

        if i==0 {
            println!("{} (N/A - no previous measurement)", x);
            continue;
        }

        let prev = sonar_values[i-1];

        if prev < x {
            count+=1;
            println!("{} (increased)", x);
        } else {
            println!("{} (decreased)", x);
        }
    }

    println!("{}", count);
}
