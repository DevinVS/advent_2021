use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Clone, Copy)]
struct BingoBoard {
    board: [[(usize, bool); 5]; 5],
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            board: [[(0, false); 5]; 5]
        }
    }

    fn set(&mut self, row: usize, col: usize, val: usize) {
        self.board[row][col].0 = val;
    }

    fn play(&mut self, val: usize) {
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col].0==val {
                    self.board[row][col].1 = true
                }
            }
        }
    }

    fn score(&self) -> Option<usize> {
        for i in 0..5 {
            if self.check_row(i) || self.check_col(i) {
                let sum: usize = self.board.iter().flatten()
                    .filter_map(|e| {
                        if e.1 {
                            None
                        } else {
                            Some(e.0)
                        }
                    })
                    .sum();

                return Some(sum);
            }
        }

        return None;
    }

    fn check_col(&self, col: usize) -> bool {
        for row in 0..5 {
            if !self.board[row][col].1 {
                return false;
            }
        }

        return true;
    }

    fn check_row(&self, row: usize) -> bool {
        for col in 0..5 {
            if !self.board[row][col].1 {
                return false;
            }
        }

        return true;
    }
}

fn main() {
    let path = "./src/giant_squid/input";
    let (guesses, mut boards) = parse_input(path);

    let mut boards_copy = boards.clone();

    // find winning board
    'outer: for guess in guesses.iter() {
        for board in boards.iter_mut() {
            board.play(*guess);

            if let Some(score) = board.score() {
                println!("{} * {} = {}",score, guess, score*guess);             break 'outer;
            }
        }
    }

    // Find losing board
    let mut win_mask = vec![false; boards_copy.len()];
    'outer_lose: for guess in guesses.iter() {
        for (i, board) in boards_copy.iter_mut().enumerate() {
            board.play(*guess);

            if let Some(score) = board.score() {
                win_mask[i] = true;

                if win_mask.iter().all(|e| *e) {
                    println!("{}", score*guess);
                    break 'outer_lose;
                }
            }
        }
    }
}

fn parse_input(path: &str) -> (Vec<usize>, Vec<BingoBoard>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    // Parse guesses
    let guess_line = lines.next().unwrap().unwrap();
    let guesses = guess_line.split(",").map(|e| {
        e.parse::<usize>().unwrap()
    }).collect::<Vec<usize>>();

    // Parse boards
    let mut boards = Vec::new();
    let mut curr_board = BingoBoard::new();

    for (i, line) in lines
        .filter(|e| !e.as_ref().unwrap().trim().is_empty())
        .enumerate() {

        if i%5==0 && i != 0 {
            boards.push(curr_board);
            curr_board = BingoBoard::new();
        }

        let row: Vec<usize> = line.unwrap().split_whitespace().map(|e| {
            e.parse::<usize>().unwrap()
        }).collect();

        for (col, num) in row.iter().enumerate() {
            curr_board.set(i%5, col, *num);
        }
    }
    boards.push(curr_board);

    (guesses, boards)
}
