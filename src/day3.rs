use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn string_to_bits(s: &str) -> Vec<u32> {
    s.trim()
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect()
}

fn vec_to_num(digits: &Vec<u32>) -> u32 {
    digits.iter().fold(0, |num, digit| (num << 1) | (digit & 1))
}

fn count_bits(mut numbers: Vec<&str>) -> Vec<u32> {
    let mut counts = string_to_bits(numbers.pop().unwrap());
    numbers.into_iter().for_each(|s| {
        let new_bits = string_to_bits(s);
        new_bits.iter().enumerate().for_each(|(i, bit)| {
            counts[i] = counts[i] + bit;
        });
    });

    counts
}

pub fn part1() {
    let input = BufReader::new(File::open("input/day3.txt").unwrap());

    let strings = input.lines().map(|s| s.unwrap()).collect::<Vec<String>>();
    let borrowed = strings.iter().map(|s| s as &str).collect();

    let counts = count_bits(borrowed);

    let final_num = vec_to_num(
        &(counts
            .iter()
            .map(|total| {
                if *total > (strings.len() as u32 / 2) {
                    1
                } else {
                    0
                }
            })
            .collect()),
    );

    let gamma = final_num;
    let epsilon = !gamma;
    let epsilon = epsilon & ((2 as u32).pow(12) - 1);

    println!("Power consumption is {}", gamma * epsilon);
}

fn part2_log(numbers: Vec<u32>, max_bit: u32) -> u32 {
    let (mut big, mut small): (Vec<u32>, Vec<u32>) = numbers
        .iter()
        .partition(|num| (**num & ((1 << (max_bit + 1)) - 1)) >= (1 << max_bit));

    for bit in (0..(max_bit)).rev() {
        if big.len() > 1 {
            let (big1, big2): (Vec<u32>, Vec<u32>) = big
                .iter()
                .partition(|num| (**num & ((1 << (bit + 1)) - 1)) >= (1 << bit));

            big = if big1.len() >= big2.len() { big1 } else { big2 };
        }

        if small.len() > 1 {
            let (small1, small2): (Vec<u32>, Vec<u32>) = small
                .iter()
                .partition(|num| (**num & ((1 << (bit + 1)) - 1)) >= (1 << bit));

            small = if small1.len() >= small2.len() {
                small2
            } else {
                small1
            };
        }

        if (big.len() == 1) && (small.len() == 1) {
            break;
        }
    }

    big[0] * small[0]
}

pub fn part2() {
    let input = BufReader::new(File::open("input/day3.txt").unwrap());

    let numbers = input
        .lines()
        .map(|s| u32::from_str_radix(s.unwrap().trim(), 2).unwrap())
        .collect::<Vec<u32>>();

    let answer = part2_log(numbers, 11);

    println!("Part2: {}", answer);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn string_to_bits_test() {
        let num = 0b110110;
        let string = "110110";

        assert_eq!(vec_to_num(&string_to_bits(string)), num);
    }

    #[test]
    fn part2_test() {
        let numbers = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        assert_eq!(part2_log(numbers, 4), 230);
    }
}
