use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::repeat,
};

#[derive(PartialEq, Debug)]
struct Line {
    pub start: (i32, i32),
    pub end: (i32, i32),
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let coords = s.split("->").map(|s| s.trim()).collect::<Vec<&str>>();
        assert_eq!(coords.len(), 2);

        if let [left, right] = coords.as_slice() {
            let start: Vec<i32> = left.split(",").map(|x| x.parse().unwrap()).collect();
            let end: Vec<i32> = right.split(",").map(|x| x.parse().unwrap()).collect();

            Self {
                start: (start[0], start[1]),
                end: (end[0], end[1]),
            }
        } else {
            Self {
                start: (0, 0),
                end: (0, 0),
            }
        }
    }
}

fn step(a: i32, b: i32) -> i32 {
    if a == b {
        0
    } else if a < b {
        1
    } else {
        -1
    }
}

struct Board {
    pub entries: HashMap<(i32, i32), i32>,
}

impl Board {
    fn draw_line(&mut self, line: &Line, diagonal: bool) {
        if !diagonal && !(line.start.0 == line.end.0 || line.start.1 == line.end.1) {
            return;
        }

        let x_step = step(line.start.0, line.end.0);
        let y_step = step(line.start.1, line.end.1);
        let dist = if x_step != 0 {
            (line.start.0 - line.end.0).abs()
        } else {
            (line.start.1 - line.end.1).abs()
        };

        for i in 0..=dist {
            let val = self
                .entries
                .entry((line.start.0 + (i * x_step), line.start.1 + (i * y_step)))
                .or_insert(0);

            *val = *val + 1;
        }
    }

    fn num_overlap(&self) -> i32 {
        self.entries.iter().filter(|(_, &val)| val > 1).count() as i32
    }
}

fn strings_to_board<'a, T: IntoIterator<Item = &'a str>>(lines: T, diagonal: bool) -> Board {
    let mut board = Board {
        entries: HashMap::new(),
    };
    lines
        .into_iter()
        .map(Line::from)
        .for_each(|line| board.draw_line(&line, diagonal));

    board
}

pub fn part1() {
    let input = BufReader::new(File::open("input/day5.txt").unwrap());

    let strings = input
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let board = strings_to_board(strings.iter().map(|s| s as &str), false);

    println!("Part1: {}", board.num_overlap());
}

pub fn part2() {
    let input = BufReader::new(File::open("input/day5.txt").unwrap());

    let strings = input
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let board = strings_to_board(strings.iter().map(|s| s as &str), true);

    println!("Part2: {}", board.num_overlap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let s = "964,133 -> 596,133";

        assert_eq!(
            Line::from(s),
            Line {
                start: (964, 133),
                end: (596, 133)
            }
        );
    }

    #[test]
    fn draw() {
        let line = Line {
            start: (0, 0),
            end: (2, 0),
        };

        let mut board = Board {
            entries: HashMap::new(),
        };

        board.draw_line(&line, false);

        let answer = vec![((0, 0), 1), ((1, 0), 1), ((2, 0), 1)];
        answer.into_iter().for_each(|(coord, val)| {
            let board_val = board.entries[&coord];
            assert_eq!(val, board_val);
        })
    }

    #[test]
    fn sample_1() {
        let sample_input = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];

        let board = strings_to_board(sample_input, false);

        assert_eq!(5, board.num_overlap());
    }

    #[test]
    fn sample_2() {
        let sample_input = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];

        let board = strings_to_board(sample_input, true);

        assert_eq!(12, board.num_overlap());
    }
}
