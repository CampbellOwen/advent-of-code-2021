use core::num;
use std::{
    fs::File,
    io::{BufReader, Read},
};

fn tick_fish(state: &mut [usize; 9]) {
    state.rotate_left(1);
    state[6] += state[8];
}

fn parse_input(s: &str) -> [usize; 9] {
    let mut fish_counts = [0; 9];
    s.split(",")
        .map(|s| s.parse::<u8>().expect("Not a number"))
        .for_each(|x| fish_counts[x as usize] += 1);

    fish_counts
}

fn simulate_days(initial_state: &mut [usize; 9], num_days: usize) {
    for _ in 0..num_days {
        tick_fish(initial_state);
    }
}

pub fn part1() {
    let mut input = BufReader::new(File::open("input/day6.txt").unwrap());

    let mut input_string = String::new();
    input
        .read_to_string(&mut input_string)
        .expect("FAiled to read line");

    let mut fish_counts = parse_input(&input_string);

    simulate_days(&mut fish_counts, 80);

    println!(
        "Number of fish after 80 days: {}",
        fish_counts.iter().sum::<usize>()
    );
}

pub fn part2() {
    let mut input = BufReader::new(File::open("input/day6.txt").unwrap());

    let mut input_string = String::new();
    input
        .read_to_string(&mut input_string)
        .expect("FAiled to read line");

    let mut fish_counts = parse_input(&input_string);

    simulate_days(&mut fish_counts, 256);

    println!(
        "Number of fish after 256 days: {}",
        fish_counts.iter().sum::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps() {
        let mut fish = [0, 1, 2, 2, 1, 0, 0, 0, 0];
        tick_fish(&mut fish);

        assert_eq!(fish, [1, 2, 2, 1, 0, 0, 0, 0, 0]);

        tick_fish(&mut fish);
        assert_eq!(fish, [2, 2, 1, 0, 0, 0, 1, 0, 1]);
    }

    #[test]
    fn run_sim_test() {
        let mut fish = [0, 1, 1, 2, 1, 0, 0, 0, 0];
        for _ in 0..80 {
            tick_fish(&mut fish);
        }

        assert_eq!(fish.iter().sum::<usize>(), 5934);
    }
}
