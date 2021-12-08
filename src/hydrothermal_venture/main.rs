use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
struct VentLine {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize
}

impl VentLine {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> VentLine {
        VentLine {
            x1,
            x2,
            y1,
            y2
        }
    }
}

struct SeaFloor {
    map: Vec<Vec<usize>>
}

impl SeaFloor {
    fn new(size: usize) -> SeaFloor {
        SeaFloor {
            map: vec![vec![0; size]; size]
        }
    }

    fn add_vent(&mut self, vent: &VentLine) {
        let mut x = vent.x1 as isize;
        let mut y = vent.y1 as isize;

        let x_dir = (vent.x2 as isize - x).signum();
        let y_dir = (vent.y2 as isize - y).signum();

        let x_dest = vent.x2 as isize + x_dir;
        let y_dest = vent.y2 as isize + y_dir;

        println!("{}, {} until {}, {}", x, y, x_dest, y_dest);

        while x != x_dest || y != y_dest {
            self.map[y as usize][x as usize] += 1;

            x += x_dir;
            y += y_dir;
        }
    }

    fn danger_count(&self) -> usize {
        self.map.iter().flatten()
            .filter(|e| **e > 1)
            .map(|_| 1)
            .sum()
    }
}

impl Display for SeaFloor {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        for row in self.map.iter() {
            for num in row.iter() {
                let s = match num {
                    0 => ".".to_string(),
                    _ => num.to_string()
                };
                fmt.write_str(&s)?;
            }
            fmt.write_str("\n")?;
        }

        Ok(())
    }
}

fn main() {
    let path = "./src/hydrothermal_venture/input";
    let vent_lines = parse_input(path);
    let max_coords = vent_lines.iter()
        .map(|e| [e.x1, e.x2, e.y1, e.y2])
        .flatten()
        .max()
        .unwrap();

    let mut sea_floor = SeaFloor::new(max_coords+1);

    for vent_line in vent_lines.iter() {
        sea_floor.add_vent(vent_line);
    }

    println!("{}", sea_floor);

    println!("Danger: {}", sea_floor.danger_count());
}

fn parse_input(path: &str) -> Vec<VentLine> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut vents = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let mut parts = line.split(" -> ");
        let mut left = parts.next().unwrap().split(",");
        let mut right = parts.next().unwrap().split(",");

        let x1 = left.next().unwrap().parse::<usize>().unwrap();
        let y1 = left.next().unwrap().parse::<usize>().unwrap();

        let x2 = right.next().unwrap().parse::<usize>().unwrap();
        let y2 = right.next().unwrap().parse::<usize>().unwrap();

        vents.push(VentLine::new(x1, y1, x2, y2));
    }

    vents
}
