use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Inst {
    X(u32),
    Y(u32),
}

fn fold_set(dots: &HashSet<(u32, u32)>, fold: Inst) -> HashSet<(u32, u32)> {
    let mut new_set = HashSet::new();

    match fold {
        Inst::X(fold) => {
            for &(x, y) in dots {
                let x = if x > fold {
                    let x = fold - (x % fold);
                    if x == fold {
                        0
                    } else {
                        x
                    }
                } else {
                    x
                };
                new_set.insert((x, y));
            }
        }
        Inst::Y(fold) => {
            for &(x, y) in dots {
                let y = if y > fold {
                    let y = fold - (y % fold);
                    if y == fold {
                        0
                    } else {
                        y
                    }
                } else {
                    y
                };
                new_set.insert((x, y));
            }
        }
    }

    new_set
}

fn parse_set(input: &str) -> HashSet<(u32, u32)> {
    let mut dots = HashSet::new();

    input
        .lines()
        .map(|line| {
            line.split_once(',')
                .expect("All lines should be the same format")
        })
        .map(|(x, y)| {
            (
                x.parse().expect("Should be parseable"),
                y.parse().expect("Should be parseable"),
            )
        })
        .for_each(|dot| {
            dots.insert(dot);
        });
    dots
}

fn parse_instruction(input: &str) -> Inst {
    let (_, num) = input.split_once('=').expect("Bad format");
    let num = num.parse().expect("Should be a number");
    if input.find('y').is_some() {
        Inst::Y(num)
    } else {
        Inst::X(num)
    }
}

fn parse_input(input: &str) -> (HashSet<(u32, u32)>, Vec<Inst>) {
    let (dots, insts) = input.trim().split_once("\n\n").expect("Input format wrong");

    let instructions = insts.lines().map(parse_instruction).collect();

    (parse_set(dots), instructions)
}

fn print_board(dots: &HashSet<(u32, u32)>) {
    let max_x = dots
        .iter()
        .map(|&(x, _)| x)
        .max()
        .expect("Should have a max");

    let max_y = dots
        .iter()
        .map(|&(_, y)| y)
        .max()
        .expect("Should have a max");

    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

pub fn part1() {
    let mut input = BufReader::new(File::open("input/day13.txt").expect("Input file should exist"));

    let mut input_string = String::new();
    input
        .read_to_string(&mut input_string)
        .expect("Should read file fine");

    let (dots, instructions) = parse_input(&input_string);
    let new_dots = fold_set(&dots, instructions[0]);

    println!("Number of dots after fold: {}", new_dots.len());
}
pub fn part2() {
    let mut input = BufReader::new(File::open("input/day13.txt").expect("Input file should exist"));

    let mut input_string = String::new();
    input
        .read_to_string(&mut input_string)
        .expect("Should read file fine");

    let (dots, instructions) = parse_input(&input_string);

    let final_board = instructions
        .iter()
        .fold(dots, |dots, &inst| fold_set(&dots, inst));

    println!("Final board: ");
    print_board(&final_board);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0";

        let dots = parse_set(input);

        assert_eq!(dots.len(), 18);
    }

    #[test]
    fn parse_instruction_test() {
        let inst = "fold along y=7";
        let inst = parse_instruction(inst);
        assert_eq!(inst, Inst::Y(7));

        let inst = "fold along x=5";
        let inst = parse_instruction(inst);
        assert_eq!(inst, Inst::X(5));
    }

    #[test]
    fn parse_input_test() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        let (dots, instructions) = parse_input(input);

        assert_eq!(dots.len(), 18);
        assert_eq!(instructions.len(), 2);
    }

    #[test]
    fn example_1() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        let (dots, instructions) = parse_input(input);

        print_board(&dots);

        assert_eq!(dots.len(), 18);

        let new_dots = fold_set(&dots, instructions[0]);
        print_board(&new_dots);

        assert_eq!(new_dots.len(), 17);

        let new_dots = fold_set(&new_dots, instructions[1]);
        print_board(&new_dots);

        assert_eq!(new_dots.len(), 16);
    }
}
