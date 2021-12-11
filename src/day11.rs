use std::{fs::File, io::BufRead, io::BufReader};

struct Board {
    data: Vec<i32>,
    width: u32,
}

impl Board {
    fn index(&self, (x, y): (u32, u32)) -> usize {
        ((y * self.width) + x) as usize
    }
    fn coord(&self, index: usize) -> (u32, u32) {
        (index as u32 % self.width, index as u32 / self.width)
    }

    fn neighbours(&self, index: usize) -> Vec<usize> {
        let masks = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let height = self.data.len() as u32 / self.width;
        let (x, y) = self.coord(index);
        masks
            .iter()
            .map(|&(a, b)| (x as i32 + a, y as i32 + b))
            .filter(|&(x, y)| x >= 0 && (x as u32) < self.width && y >= 0 && (y as u32) < height)
            .map(|(x, y)| self.index((x as u32, y as u32)))
            .filter(|&i| self.data[i] >= 0)
            .collect()
    }

    fn step(&mut self) -> usize {
        let mut num_flashes = 0;
        // Stage 1 - Increase power level
        for i in 0..self.data.len() {
            self.data[i] += 1;
        }

        let mut check_9s = true;
        while check_9s {
            check_9s = false;

            for i in 0..self.data.len() {
                if self.data[i] > 9 {
                    // Flash
                    check_9s = true; // check if more >9s created

                    self.data[i] = -1; // Sentinel
                    num_flashes += 1;

                    let neighbours = self.neighbours(i);
                    neighbours.iter().for_each(|&j| self.data[j] += 1);
                }
            }
        }
        // Reset flashed
        for i in 0..self.data.len() {
            if self.data[i] == -1 {
                self.data[i] = 0;
            }
        }

        num_flashes
    }

    fn all_flashing(&self) -> bool {
        self.data.iter().all(|&x| x == 0)
    }
}

pub fn part1() {
    let input = BufReader::new(File::open("input/day11.txt").unwrap());

    let width = 10;
    let height = 10;
    let mut data: Vec<i32> = Vec::new();
    data.reserve(width * height);

    input.lines().for_each(|line| {
        let line = line.expect("invalid line");
        line.trim()
            .chars()
            .map(|c| c.to_digit(10).expect("invalid digit") as i32)
            .for_each(|x| data.push(x));
    });

    let mut board = Board {
        data,
        width: width as u32,
    };

    let num_flashes = (0..100).fold(0, |count, _| count + board.step());

    println!("Number of flashes is {}", num_flashes);
}
pub fn part2() {
    let input = BufReader::new(File::open("input/day11.txt").unwrap());

    let width = 10;
    let height = 10;
    let mut data: Vec<i32> = Vec::new();
    data.reserve(width * height);

    input.lines().for_each(|line| {
        let line = line.expect("invalid line");
        line.trim()
            .chars()
            .map(|c| c.to_digit(10).expect("invalid digit") as i32)
            .for_each(|x| data.push(x));
    });

    let mut board = Board {
        data,
        width: width as u32,
    };

    let mut step = 0;
    while !board.all_flashing() {
        step += 1;
        board.step();
    }

    println!("All flashing after step {}", step);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_test() {
        let initial_data = vec![
            5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1, 7,
            3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2, 4, 6,
            4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6, 8, 4, 8,
            5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
        ];
        let width = 10;

        let mut board = Board {
            data: initial_data,
            width,
        };

        board.step();
        assert_eq!(
            board.data,
            vec![
                6, 5, 9, 4, 2, 5, 4, 3, 3, 4, 3, 8, 5, 6, 9, 6, 5, 8, 2, 2, 6, 3, 7, 5, 6, 6, 7, 2,
                8, 4, 7, 2, 5, 2, 4, 4, 7, 2, 5, 7, 7, 4, 6, 8, 4, 9, 6, 5, 8, 9, 5, 2, 7, 8, 6, 3,
                5, 7, 5, 6, 3, 2, 8, 7, 9, 5, 2, 8, 3, 2, 7, 9, 9, 3, 9, 9, 2, 2, 4, 5, 5, 9, 5, 7,
                9, 5, 9, 6, 6, 5, 6, 3, 9, 4, 8, 6, 2, 6, 3, 7,
            ]
        );

        board.step();
        assert_eq!(
            board.data,
            vec![
                8, 8, 0, 7, 4, 7, 6, 5, 5, 5, 5, 0, 8, 9, 0, 8, 7, 0, 5, 4, 8, 5, 9, 7, 8, 8, 9, 6,
                0, 8, 8, 4, 8, 5, 7, 6, 9, 6, 0, 0, 8, 7, 0, 0, 9, 0, 8, 8, 0, 0, 6, 6, 0, 0, 0, 8,
                8, 9, 8, 9, 6, 8, 0, 0, 0, 0, 5, 9, 4, 3, 0, 0, 0, 0, 0, 0, 7, 4, 5, 6, 9, 0, 0, 0,
                0, 0, 0, 8, 7, 6, 8, 7, 0, 0, 0, 0, 6, 8, 4, 8,
            ]
        );
    }
}
