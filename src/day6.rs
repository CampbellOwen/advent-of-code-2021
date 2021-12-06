use core::num;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

fn update_state(fish: &mut Vec<u8>) {
    let mut num_new = 0;

    for i in 0..fish.len() {
        let val = fish[i];
        if val == 0 {
            fish[i] = 6;
            num_new += 1;
        } else {
            fish[i] = val - 1;
        }
    }
    for _ in 0..num_new {
        fish.push(8);
    }
}

fn run_sim(fish: &mut Vec<u8>, num_days: usize) {
    for _ in 0..num_days {
        update_state(fish);
    }
}

pub fn part1() {
    let mut input = BufReader::new(File::open("input/day6.txt").unwrap());

    let mut input_string = String::new();
    input
        .read_to_string(&mut input_string)
        .expect("FAiled to read line");

    let mut initial_state: Vec<u8> = input_string
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    run_sim(&mut initial_state, 80);

    println!("Number of fish after 80 days: {}", initial_state.len());
}

fn run_state_efficient(state: &mut HashMap<u8, usize>) {
    let num_0 = *state.get(&0).unwrap_or(&0);
    for i in 0..8 {
        let upper_val = *state.get(&(i + 1)).unwrap_or(&0);
        state.insert(i, upper_val);
    }
    state.insert(8, num_0);

    let num_6 = state.entry(6).or_insert(0);
    *num_6 = *num_6 + num_0;
}

fn run_sim_efficient(state: &mut HashMap<u8, usize>, num_days: usize) {
    for _ in 0..num_days {
        run_state_efficient(state);
    }
}

pub fn part2() {
    let mut input = BufReader::new(File::open("input/day6.txt").unwrap());

    let mut input_string = String::new();
    input
        .read_to_string(&mut input_string)
        .expect("FAiled to read line");

    let initial_state: Vec<u8> = input_string
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut state = HashMap::new();
    for i in 0..=8 {
        state.insert(i, initial_state.iter().filter(|&x| *x == (i as u8)).count());
    }

    run_sim_efficient(&mut state, 256);

    let num_fish: usize = state.into_values().sum();

    println!("Number of fish after 80 days: {}", num_fish);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps() {
        let mut fish = vec![3, 4, 3, 1, 2];
        update_state(&mut fish);
        assert_eq!(fish, vec![2, 3, 2, 0, 1]);

        update_state(&mut fish);
        assert_eq!(fish, vec![1, 2, 1, 6, 0, 8]);
    }

    #[test]
    fn run_sim_test() {
        let mut initial = vec![3, 4, 3, 1, 2];
        run_sim(&mut initial, 80);

        assert_eq!(initial.len(), 5934);
    }

    #[test]
    fn run_efficient_steps() {
        let mut fish = HashMap::from([
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 1),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
        ]);

        run_state_efficient(&mut fish);
        assert_eq!(fish[&0], 1);
        assert_eq!(fish[&1], 1);
        assert_eq!(fish[&2], 2);
        assert_eq!(fish[&3], 1);
        assert_eq!(fish[&4], 0);
        assert_eq!(fish[&5], 0);
        assert_eq!(fish[&6], 0);
        assert_eq!(fish[&7], 0);
        assert_eq!(fish[&8], 0);

        run_state_efficient(&mut fish);
        assert_eq!(fish[&0], 1);
        assert_eq!(fish[&1], 2);
        assert_eq!(fish[&2], 1);
        assert_eq!(fish[&3], 0);
        assert_eq!(fish[&4], 0);
        assert_eq!(fish[&5], 0);
        assert_eq!(fish[&6], 1);
        assert_eq!(fish[&7], 0);
        assert_eq!(fish[&8], 1);
    }
}
