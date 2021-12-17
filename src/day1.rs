use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1() {
    let mut input = BufReader::new(File::open("input/day1.txt").expect("day 1 input missing"));

    let mut first_line = String::new();
    input
        .read_line(&mut first_line)
        .expect("Error reading first line");

    println!("First line: {}", first_line);
    let first_num: i32 = first_line.trim().parse().expect("First line not a number");
    let init = (0, first_num);

    let (total, _) = input.lines().fold(init, |(total, prev_num), line| {
        let line = line.expect("Not a valid line?");

        let number: i32 = line.trim().parse().expect("Line not a number :(");

        (if number > prev_num { total + 1 } else { total }, number)
    });

    println!("Total increasing numbers: {}", total);
}

pub fn part2() {
    let input = BufReader::new(File::open("input/day1.txt").expect("day 1 input missing"));

    let lines: Vec<i32> = input
        .lines()
        .map(|line| line.unwrap().trim().parse::<i32>().unwrap())
        .collect();

    let offsets = [0, 1, 2];

    let mut prev_sum = 0;
    let mut total = -1;
    for i in 0..lines.len() - 2 {
        let sum: i32 = offsets.iter().map(|offset| lines[i + offset]).sum();

        if sum > prev_sum {
            total += 1;
        }
        prev_sum = sum;
    }

    println!("Total increasing groups of 3: {}", total);
}
