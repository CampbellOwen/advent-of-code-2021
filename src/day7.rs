use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_input(s: &str) -> Vec<i32> {
    s.split(',').map(|s| s.parse().unwrap()).collect()
}

fn calculate_cost(x: i32, y: i32, part_1: bool) -> i32 {
    if part_1 {
        (x - y).abs()
    } else {
        let dist = (x - y).abs();

        let cost = (dist as f32 / 2.0) * (1 + dist) as f32;

        cost as i32
    }
}

fn optimal_position(positions: &[i32], part_1: bool) -> (i32, i32) {
    let &min = positions.iter().min().unwrap();
    let &max = positions.iter().max().unwrap();

    (min..=max)
        .map(|pos| {
            (
                pos,
                positions
                    .iter()
                    .map(|&p| calculate_cost(p, pos, part_1))
                    .sum::<i32>(),
            )
        })
        .min_by(|(_, cost1), (_, cost2)| cost1.cmp(cost2))
        .unwrap()
}

pub fn part1() {
    let mut input = BufReader::new(File::open("input/day7.txt").unwrap());

    let mut input_str = String::new();
    input
        .read_line(&mut input_str)
        .expect("Read first line of input");

    let numbers = parse_input(&input_str);

    let (position, cost) = optimal_position(&numbers, true);

    println!("Fuel cost to get to {} is {}", position, cost);
}

pub fn part2() {
    let mut input = BufReader::new(File::open("input/day7.txt").unwrap());

    let mut input_str = String::new();
    input
        .read_line(&mut input_str)
        .expect("Read first line of input");

    let numbers = parse_input(&input_str);

    let (position, cost) = optimal_position(&numbers, false);

    println!("Fuel cost to get to {} is {}", position, cost);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_cost_test() {
        let x = 16;
        let y = 5;

        assert_eq!(calculate_cost(x, y, true), 11);
        assert_eq!(calculate_cost(x, y, false), 66);
    }

    #[test]
    fn optimal_position_test() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let (position, cost) = optimal_position(&input, true);

        assert_eq!(position, 2);
        assert_eq!(cost, 37);
    }

    #[test]
    fn optimal_position_test_2() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let (position, cost) = optimal_position(&input, false);

        assert_eq!(position, 5);
        assert_eq!(cost, 168);
    }
}
