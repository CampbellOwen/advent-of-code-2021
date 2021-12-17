use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Instruction {
    fn parse(s: &str) -> Option<Instruction> {
        let mut parts = s.split(' ');
        let inst = parts.next()?;
        let magnitude = parts.next()?.parse::<i32>().ok()?;

        match inst {
            "forward" => Some(Instruction::Forward(magnitude)),
            "down" => Some(Instruction::Down(magnitude)),
            "up" => Some(Instruction::Up(magnitude)),
            _ => None,
        }
    }
}

type State1 = (i32, i32);

fn execute_1(state: State1, inst: Instruction) -> State1 {
    let (horiz, vert) = state;
    match inst {
        Instruction::Up(n) => (horiz, vert - n),
        Instruction::Down(n) => (horiz, vert + n),
        Instruction::Forward(n) => (horiz + n, vert),
    }
}

fn execute_series<I>(instructions: I) -> State1
where
    I: IntoIterator<Item = String>,
{
    let initial_state = (0, 0);
    instructions
        .into_iter()
        .map(|line| Instruction::parse(line.trim()))
        .flatten()
        .fold(initial_state, execute_1)
}

pub fn part1() {
    let input = BufReader::new(File::open("input/day2.txt").unwrap());

    let (h, v) = execute_series(input.lines().filter_map(|line| line.ok()));

    let answer = h * v;

    println!("Day2 part 1 answer is {:?}", answer);
}

type State2 = (i32, i32, i32);

fn execute_2(state: State2, inst: Instruction) -> State2 {
    let (h, v, aim) = state;
    match inst {
        Instruction::Up(n) => (h, v, aim - n),
        Instruction::Down(n) => (h, v, aim + n),
        Instruction::Forward(n) => (h + n, v + (n * aim), aim),
    }
}

fn execute_series_2<I>(instructions: I) -> State2
where
    I: IntoIterator<Item = String>,
{
    let initial_state = (0, 0, 0);
    instructions
        .into_iter()
        .map(|line| Instruction::parse(line.trim()))
        .flatten()
        .fold(initial_state, execute_2)
}

pub fn part2() {
    let input = BufReader::new(File::open("input/day2.txt").unwrap());

    let (h, v, _) = execute_series_2(input.lines().filter_map(|line| line.ok()));

    let answer = h * v;

    println!("Day2 part 2 answer is {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foward() {
        let state = (0, 0);
        let inst = Instruction::Forward(12);

        assert_eq!(execute_1(state, inst), (12, 0));
    }

    #[test]
    fn up() {
        let state = (0, 0);
        let inst = Instruction::Up(12);

        assert_eq!(execute_1(state, inst), (0, -12));
    }
    #[test]
    fn down() {
        let state = (0, 0);
        let inst = Instruction::Down(12);

        assert_eq!(execute_1(state, inst), (0, 12));
    }

    #[test]
    fn part1() {
        let input = vec![
            "forward 5",
            "down 5   ",
            "forward 8",
            "up 3",
            "down 8   ",
            "forward 2",
        ];

        let (h, v) = execute_series(input.into_iter().map(|s| String::from(s)));
        assert_eq!(h, 15);
        assert_eq!(v, 10);
    }

    #[test]
    fn part2() {
        let input = vec![
            "forward 5",
            "down 5   ",
            "forward 8",
            "up 3",
            "down 8   ",
            "forward 2",
        ];

        let (h, v, _) = execute_series_2(input.into_iter().map(|s| String::from(s)));
        assert_eq!(h, 15);
        assert_eq!(v, 60);
    }
}
