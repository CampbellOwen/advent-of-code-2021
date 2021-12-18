use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufReader, Read},
};

struct Cave {
    data: Vec<u8>,
    width: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PathEntry {
    pos: (usize, usize),
    cost: usize,
    prev: Option<(usize, usize)>,
}

impl PartialOrd for PathEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for PathEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl Cave {
    fn index(&self, (x, y): (usize, usize)) -> usize {
        ((y * self.width) + x) as usize
    }
    fn coord(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }
    fn neighbours(&self, coord: (usize, usize), tiled: bool) -> Vec<(usize, usize)> {
        let (x, y) = coord;

        let mut neighbours = Vec::new();

        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1))
        }
        if x < (self.width - 1) || (tiled && x < (5 * self.width - 1)) {
            neighbours.push((x + 1, y))
        }
        if y < (self.width - 1) || (tiled && y < (5 * self.width - 1)) {
            neighbours.push((x, y + 1))
        }

        neighbours
    }

    fn heuristic_cost(&self, pos: (usize, usize), goal: (usize, usize)) -> usize {
        let (pos_x, pos_y) = pos;
        let (goal_x, goal_y) = goal;

        ((goal_x as i64 - pos_x as i64).abs() + (goal_y as i64 - pos_y as i64).abs()) as usize
    }

    fn risk(&self, coord: (usize, usize), tiled: bool) -> usize {
        if !tiled {
            let index = self.index(coord);

            self.data[index] as usize
        } else {
            let (x, y) = coord;
            let add_x = x / self.width;
            let x = x % self.width;

            let add_y = y / self.width;
            let y = y % self.width;

            let risk = self.data[self.index((x, y))] as usize + add_x + add_y;

            if risk > 9 {
                risk - 9
            } else {
                risk
            }
        }
    }

    fn find_path(&self, start: (usize, usize), end: (usize, usize), tiled: bool) -> Vec<PathEntry> {
        let mut path = HashMap::from([(
            start,
            PathEntry {
                pos: start,
                cost: 0,
                prev: None,
            },
        )]);

        let mut frontier: BinaryHeap<Reverse<PathEntry>> =
            BinaryHeap::from_iter(self.neighbours(start, tiled).iter().map(|&coord| {
                Reverse(PathEntry {
                    pos: coord,
                    cost: self.risk(coord, tiled),
                    prev: Some(start),
                })
            }));

        while !frontier.is_empty() && !path.contains_key(&end) {
            let mut pos = frontier.pop().unwrap().0;

            pos.cost = self.risk(pos.pos, tiled) + path[&pos.prev.unwrap()].cost;

            let entry = path.entry(pos.pos).or_insert(pos);
            if pos.cost <= entry.cost {
                *entry = pos;
            } else {
                continue;
            }

            self.neighbours(pos.pos, tiled)
                .iter()
                .map(|&coord| {
                    let cost = pos.cost + self.risk(coord, tiled);

                    PathEntry {
                        pos: coord,
                        cost,
                        prev: Some(pos.pos),
                    }
                })
                .for_each(|entry| {
                    if !path.contains_key(&entry.pos) || entry.cost < path[&entry.pos].cost {
                        let priority_cost = entry.cost + self.heuristic_cost(entry.pos, end);

                        frontier.push(Reverse(PathEntry {
                            pos: entry.pos,
                            cost: priority_cost,
                            prev: entry.prev,
                        }))
                    }
                })
        }

        let last_entry = path[&end];
        let mut linear_path = vec![PathEntry {
            pos: last_entry.pos,
            cost: self.risk(last_entry.pos, tiled),
            prev: last_entry.prev,
        }];

        while linear_path.last().unwrap().pos != start {
            let entry = path[&linear_path.last().unwrap().prev.unwrap()];
            let cost = self.risk(entry.pos, tiled);
            linear_path.push(PathEntry {
                pos: entry.pos,
                cost,
                prev: entry.prev,
            });
        }

        linear_path.iter().rev().copied().collect()
    }
}

fn parse_cave(s: &str, width: usize) -> Cave {
    let mut data: Vec<u8> = Vec::new();
    data.reserve(width * width);

    s.lines().for_each(|line| {
        line.trim()
            .chars()
            .map(|c| c.to_digit(10).expect("invalid digit") as u8)
            .for_each(|x| data.push(x));
    });

    Cave { data, width }
}

fn cost(path: &[PathEntry]) -> usize {
    path.iter().fold(0, |score, &entry| score + entry.cost) - path[0].cost
}

pub fn part1() {
    let mut input_buf = BufReader::new(File::open("input/day15.txt").expect("Input should exist"));

    let mut input = String::new();
    input_buf.read_to_string(&mut input).expect("Read input");

    let cave = parse_cave(&input, 100);

    let path = cave.find_path((0, 0), (99, 99), false);
    let path_risk = cost(&path);

    println!("Risk of safest path: {}", path_risk);
}
pub fn part2() {
    let mut input_buf = BufReader::new(File::open("input/day15.txt").expect("Input should exist"));

    let mut input = String::new();
    input_buf.read_to_string(&mut input).expect("Read input");

    let cave = parse_cave(&input, 100);

    let path = cave.find_path((0, 0), (499, 499), true);
    let path_risk = cost(&path);

    println!("Risk of safest path: {}", path_risk);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        let cave = Cave {
            data: vec![
                1, 1, 6, 3, 7, 5, 1, 7, 4, 2, 1, 3, 8, 1, 3, 7, 3, 6, 7, 2, 2, 1, 3, 6, 5, 1, 1, 3,
                2, 8, 3, 6, 9, 4, 9, 3, 1, 5, 6, 9, 7, 4, 6, 3, 4, 1, 7, 1, 1, 1, 1, 3, 1, 9, 1, 2,
                8, 1, 3, 7, 1, 3, 5, 9, 9, 1, 2, 4, 2, 1, 3, 1, 2, 5, 4, 2, 1, 6, 3, 9, 1, 2, 9, 3,
                1, 3, 8, 5, 2, 1, 2, 3, 1, 1, 9, 4, 4, 5, 8, 1,
            ],
            width: 10,
        };

        let path = cave.find_path((0, 0), (9, 9), false);

        assert_eq!(cost(&path), 40);
    }

    #[test]
    fn example_2() {
        let cave = Cave {
            data: vec![
                1, 1, 6, 3, 7, 5, 1, 7, 4, 2, 1, 3, 8, 1, 3, 7, 3, 6, 7, 2, 2, 1, 3, 6, 5, 1, 1, 3,
                2, 8, 3, 6, 9, 4, 9, 3, 1, 5, 6, 9, 7, 4, 6, 3, 4, 1, 7, 1, 1, 1, 1, 3, 1, 9, 1, 2,
                8, 1, 3, 7, 1, 3, 5, 9, 9, 1, 2, 4, 2, 1, 3, 1, 2, 5, 4, 2, 1, 6, 3, 9, 1, 2, 9, 3,
                1, 3, 8, 5, 2, 1, 2, 3, 1, 1, 9, 4, 4, 5, 8, 1,
            ],
            width: 10,
        };

        let neighbours = cave.neighbours((9, 0), true);

        assert_eq!(neighbours.len(), 3);
        assert!(neighbours.contains(&(8, 0)));
        assert!(neighbours.contains(&(9, 1)));
        assert!(neighbours.contains(&(10, 0)));

        assert_eq!(cave.risk((10, 0), true), 2);
        assert_eq!(cave.risk((20, 0), true), 3);
        assert_eq!(cave.risk((30, 0), true), 4);
        assert_eq!(cave.risk((40, 0), true), 5);

        assert_eq!(cave.risk((10, 10), true), 3);
        assert_eq!(cave.risk((49, 49), true), 9);
        assert_eq!(cave.risk((48, 49), true), 7);

        let path = cave.find_path((0, 0), (49, 49), true);

        assert_eq!(cost(&path), 315);
    }
}
