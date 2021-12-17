use core::num;
use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    io::{BufRead, Read},
};

struct CaveSystem {
    paths: HashMap<String, Vec<String>>,
}

impl From<&str> for CaveSystem {
    fn from(s: &str) -> Self {
        let mut state = HashMap::new();
        s.lines()
            .filter_map(|line| line.split_once('-'))
            .for_each(|(start, end)| {
                let start = start.to_string();
                let end = end.to_string();

                let entry = state.entry(start.clone()).or_insert_with(Vec::new);

                if !entry.contains(&end) && end != "start" && start != "end" {
                    entry.push(end.clone());
                }

                let entry = state.entry(end.clone()).or_insert_with(Vec::new);
                if !entry.contains(&start) && start != "start" && end != "end" {
                    entry.push(start);
                }
            });

        CaveSystem { paths: state }
    }
}

impl CaveSystem {
    fn paths(&self, duplicate_once: bool) -> Vec<Vec<String>> {
        let mut paths = Vec::new();

        let mut next_paths = vec![self.paths["start"].iter().peekable()];

        let mut path = vec!["start".to_string()];

        let mut duplicate = None;
        while !next_paths.is_empty() {
            let last_item = next_paths.last_mut().unwrap();
            let has_some = last_item.peek().is_some();

            if has_some {
                let node = last_item.next().expect("Just checked that it's some");

                let duplicate_small = &node.to_lowercase() == node && path.contains(node);

                if !duplicate_small || (duplicate_once && duplicate.is_none()) {
                    if duplicate_once && duplicate_small && duplicate.is_none() {
                        duplicate = Some(node.clone());
                    }
                    path.push(node.clone());

                    if node == "end" {
                        paths.push(path.clone());
                        path.pop();
                    } else {
                        let next = self.paths.get(node).expect("Paths are bidirectional");
                        next_paths.push(next.iter().peekable());
                    }
                }
            } else {
                next_paths.pop();
                let node = path.pop().unwrap();
                if let Some(dup) = &duplicate {
                    if dup == &node {
                        duplicate = None;
                    }
                }
            }
        }

        paths
    }
}

pub fn part1() {
    let mut input = BufReader::new(File::open("input/day12.txt").expect("Input file should exist"));

    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Should read input fine");

    let cave_system = CaveSystem::from(&input_str as &str);
    let num_paths = cave_system.paths(false).len();

    println!("Number of paths: {}", num_paths);
}

pub fn part2() {
    let mut input = BufReader::new(File::open("input/day12.txt").expect("Input file should exist"));

    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Should read input fine");

    let cave_system = CaveSystem::from(&input_str as &str);
    let num_paths = cave_system.paths(true).len();

    println!("Number of paths: {}", num_paths);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_paths_1() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        let caves = CaveSystem::from(input);

        let paths = caves.paths(false);
        println!("{:?}", paths);
        assert_eq!(paths.len(), 10);
    }

    #[test]
    fn number_paths_1_with_dup() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        let caves = CaveSystem::from(input);

        let paths = caves.paths(true);
        println!("{:?}", paths);
        assert_eq!(paths.len(), 36);
    }

    #[test]
    fn number_paths_2() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

        let caves = CaveSystem::from(input);

        let paths = caves.paths(false);
        assert_eq!(paths.len(), 19);
    }

    #[test]
    fn number_paths_3() {
        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

        let caves = CaveSystem::from(input);

        let paths = caves.paths(false);
        assert_eq!(paths.len(), 226);
    }
}
