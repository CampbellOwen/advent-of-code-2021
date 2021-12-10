use std::{fs::File, io::BufRead, io::BufReader};

struct HeightMap {
    data: Vec<u8>,
    width: u32,
}

impl HeightMap {
    fn index(&self, (x, y): (u32, u32)) -> usize {
        ((y * self.width) + x) as usize
    }
    fn coord(&self, index: usize) -> (u32, u32) {
        (index as u32 % self.width, index as u32 / self.width)
    }
    fn low_points(&self) -> Vec<(u32, u32)> {
        let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let map_height = self.data.len() / self.width as usize;

        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, &height)| {
                let (x, y) = self.coord(i);

                let all_lower = directions
                    .iter()
                    .filter_map(|&(dx, dy)| {
                        let (x, y) = (x as i32 + dx, y as i32 + dy);
                        if x < 0 || x >= self.width as i32 || y < 0 || y >= map_height as i32 {
                            return None;
                        }

                        let coord = (x as u32, y as u32);
                        let val = self.data[self.index(coord)];
                        if val <= height {
                            Some(false)
                        } else {
                            Some(true)
                        }
                    })
                    .all(|x| x);

                if all_lower {
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect()
    }
    fn risk_level(&self) -> u32 {
        let low_points = self.low_points();
        low_points
            .iter()
            .map(|&coord| self.index(coord))
            .map(|i| self.data[i])
            .map(|height| (height + 1) as u32)
            .sum()
    }

    fn neighbours(&self, coord: (u32, u32)) -> Vec<(u32, u32)> {
        let height = self.data.len() as u32 / self.width;
        let (x, y) = coord;

        let mut neighbours = Vec::new();

        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1))
        }
        if x < (self.width - 1) {
            neighbours.push((x + 1, y))
        }
        if y < (height - 1) {
            neighbours.push((x, y + 1))
        }

        neighbours
    }

    fn find_basin(&self, low_point: (u32, u32)) -> Vec<(u32, u32)> {
        let mut basin = Vec::new();

        let mut frontier = vec![low_point];

        while !frontier.is_empty() {
            let coord = frontier.pop().unwrap();
            if basin.contains(&coord) {
                continue;
            }
            basin.push(coord);
            self.neighbours(coord)
                .iter()
                .filter(|&&new_coord| {
                    !basin.contains(&new_coord) && self.data[self.index(new_coord)] != 9
                })
                .for_each(|&coord| frontier.push(coord));
        }

        basin
    }
}

pub fn part1() {
    let input = BufReader::new(File::open("input/day9.txt").expect("No input found"));

    let width = 100;
    let height = 100;
    let mut data: Vec<u8> = Vec::new();
    data.reserve(width * height);

    input.lines().for_each(|line| {
        let line = line.expect("invalid line");
        line.trim()
            .chars()
            .map(|c| c.to_digit(10).expect("invalid digit") as u8)
            .for_each(|x| data.push(x));
    });

    let board = HeightMap {
        data,
        width: width as u32,
    };

    println!("Risk level: {}", board.risk_level());
}

pub fn part2() {
    let input = BufReader::new(File::open("input/day9.txt").expect("No input found"));

    let width = 100;
    let height = 100;
    let mut data: Vec<u8> = Vec::new();
    data.reserve(width * height);

    input.lines().for_each(|line| {
        let line = line.expect("invalid line");
        line.trim()
            .chars()
            .map(|c| c.to_digit(10).expect("invalid digit") as u8)
            .for_each(|x| data.push(x));
    });

    let board = HeightMap {
        data,
        width: width as u32,
    };

    let mut basin_sizes = board
        .low_points()
        .iter()
        .map(|&coord| board.find_basin(coord).len())
        .collect::<Vec<usize>>();
    basin_sizes.sort();
    let product = basin_sizes
        .iter()
        .rev()
        .take(3)
        .map(|&x| x)
        .reduce(|product, num| product * num)
        .expect("product failed");

    println!("Product of 3 largest basins: {}", product);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_test() {
        let heightmap = HeightMap {
            data: Vec::new(),
            width: 10,
        };
        let xy = (5, 5);
        assert_eq!(heightmap.index(xy), 55);
    }

    #[test]
    fn coord_test() {
        let heightmap = HeightMap {
            data: Vec::new(),
            width: 10,
        };

        let index = 55;
        assert_eq!(heightmap.coord(index), (5, 5));
    }

    #[test]
    fn low_point_test() {
        let data = vec![
            2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
            2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
        ];
        let heightmap = HeightMap { data, width: 10 };
        let low_points = heightmap.low_points();
        assert_eq!(low_points.len(), 4);
    }

    #[test]
    fn risk_level_test() {
        let data = vec![
            2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
            2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
        ];
        let heightmap = HeightMap { data, width: 10 };

        let risk_level = heightmap.risk_level();
        assert_eq!(risk_level, 15);
    }

    #[test]
    fn find_basin_test() {
        let data = vec![
            2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
            2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
        ];
        let heightmap = HeightMap { data, width: 10 };

        let low_point = (1, 0);
        assert_eq!(heightmap.find_basin(low_point).len(), 3);

        let low_point = (9, 0);
        assert_eq!(heightmap.find_basin(low_point).len(), 9);

        let low_point = (2, 2);
        assert_eq!(heightmap.find_basin(low_point).len(), 14);

        let low_point = (8, 4);
        assert_eq!(heightmap.find_basin(low_point).len(), 9);
    }
}
