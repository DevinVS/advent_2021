use std::fs::File;
use std::io::{BufReader, BufRead};

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl Command {
    fn from_string(s: &str) -> Command {
        let mut parts = s.trim().split(" ");
        let cmd = parts.next().unwrap();
        let val = parts.next().unwrap().parse::<usize>().unwrap();

        match cmd {
            "forward" => Command::Forward(val),
            "down" => Command::Down(val),
            "up" => Command::Up(val),
            _ => {
                println!("{}", cmd);
                unreachable!()
            }
        }
    }
}

struct Submarine {
    horizontal: usize,
    depth: usize
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            horizontal: 0,
            depth: 0
        }
    }

    fn run(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(x) => self.horizontal += x,
            Command::Up(x) => self.depth -= x,
            Command::Down(x) => self.depth += x
        }
    }
}

struct SubmarineV2 {
    horizontal: usize,
    depth: usize,
    aim: usize
}

impl SubmarineV2 {
    fn new() -> SubmarineV2 {
        SubmarineV2 {
            horizontal: 0,
            depth: 0,
            aim: 0
        }
    }

    fn run(&mut self, cmd: &Command) {
        match cmd {
            Command::Down(x) => self.aim += x,
            Command::Up(x) => self.aim -= x,
            Command::Forward(x) => {
                self.horizontal += x;
                self.depth += self.aim * x;
            }
        }
    }
}

fn main() {
    let path = "./src/dive/input";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut command_buffer = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        command_buffer.push(Command::from_string(&line));
    }

    let mut submarine = Submarine::new();
    for cmd in command_buffer.iter() {
        submarine.run(&cmd);
    }

    println!("{}", submarine.depth*submarine.horizontal);

    let mut submarine = SubmarineV2::new();
    for cmd in command_buffer.iter() {
        submarine.run(&cmd);
    }

    println!("{}", submarine.depth*submarine.horizontal);
}
