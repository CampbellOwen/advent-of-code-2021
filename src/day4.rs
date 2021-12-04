use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug)]
struct Board {
    pub numbers: Vec<i32>,
    pub size: i32,
}

impl Board {
    pub fn mark_num(&mut self, num: i32) {
        let indices: Vec<usize> = self
            .numbers
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x == num { Some(i) } else { None })
            .collect();

        for i in indices {
            self.numbers[i] = -1;
        }
    }

    pub fn is_won(&self) -> bool {
        for y in 0..self.size {
            let mut row_won = true;
            let mut col_won = true;
            for x in 0..self.size {
                let i = (y * self.size) + x;
                if self.numbers[i as usize] != -1 {
                    row_won = false;
                }

                let i = (x * self.size) + y;
                if self.numbers[i as usize] != -1 {
                    col_won = false;
                }
            }
            if row_won || col_won {
                return true;
            }
        }

        false
    }

    pub fn value(&self) -> i32 {
        self.numbers.iter().filter(|&&x| x >= 0).sum()
    }
}

fn parse_board(s: &str) -> Board {
    let numbers = s
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let size = (numbers.len() as f64).sqrt() as i32;

    Board { numbers, size }
}

fn parse_all_boards(s: &str) -> Vec<Board> {
    s.trim().split("\n\n").map(parse_board).collect()
}

pub fn part1() {
    let mut input = BufReader::new(File::open("input/day4.txt").unwrap());

    let mut draws = String::new();
    input
        .read_line(&mut draws)
        .expect("Reading first line as draws");

    let draws = draws
        .split(',')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let mut boards = String::new();
    input
        .read_to_string(&mut boards)
        .expect("Reading rest of input into boards");

    let mut boards = parse_all_boards(&boards);

    for draw in draws {
        for board in &mut boards {
            board.mark_num(draw);

            if board.is_won() {
                println!("Winning score: {}", board.value() * draw);
                return;
            }
        }
    }
}

pub fn part2() {
    let mut input = BufReader::new(File::open("input/day4.txt").unwrap());

    let mut draws = String::new();
    input
        .read_line(&mut draws)
        .expect("Reading first line as draws");

    let draws = draws
        .split(',')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let mut boards = String::new();
    input
        .read_to_string(&mut boards)
        .expect("Reading rest of input into boards");

    let mut boards = parse_all_boards(&boards);

    let num_boards = boards.len();
    let mut num_winners = 0;

    for draw in draws {
        for board in &mut boards {
            if !board.is_won() {
                board.mark_num(draw);

                if board.is_won() {
                    num_winners = num_winners + 1;

                    if num_winners == num_boards {
                        println!("Winning score: {}", board.value() * draw);
                        return;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_board() {
        let board = Board {
            numbers: vec![-1, -1, -1, 1, 2, 3, 4, 5, 6],
            size: 3,
        };

        assert!(board.is_won());

        let board = Board {
            numbers: vec![1, 2, 3, -1, -1, -1, 4, 5, 6],
            size: 3,
        };

        assert!(board.is_won());

        let board = Board {
            numbers: vec![1, 2, 3, 4, 5, 6, -1, -1, -1],
            size: 3,
        };

        assert!(board.is_won());

        let board = Board {
            numbers: vec![-1, 2, 3, -1, 5, 6, -1, 8, 9],
            size: 3,
        };

        assert!(board.is_won());
    }

    #[test]
    fn mark_board() {
        let mut board = Board {
            numbers: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            size: 3,
        };
        assert!(!board.is_won());

        board.mark_num(1);
        assert!(!board.is_won());

        board.mark_num(2);
        assert!(!board.is_won());

        board.mark_num(3);
        assert!(board.is_won());
    }
}
