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

    println!("Increasing Count: {}", count);
    let mut window_count = 0;
    let mut last_sum = None;

    // Now calculate the same except using a 3 value window
    for i in 0..sonar_values.len()-2 {
        let window_sum: u32 = sonar_values[i..i+3].iter().sum();

        if let Some(prev) = last_sum {
            if prev < window_sum {
                window_count += 1;
                println!("{} (increased)", window_sum);
            } else {
                println!("{} (decreased)", window_sum);
            }
        } else {
            println!("{} (N/A - no previous sum)", window_sum);
        }

        last_sum = Some(window_sum);
    }

    println!("Window Count: {}", window_count);
}
