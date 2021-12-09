use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    fmt::Error,
    fs::File,
    io::BufRead,
    io::BufReader,
    ops::{BitAnd, BitOr},
};

fn split_input(s: &str) -> (&str, &str) {
    let mut it = s.split('|');
    (it.next().unwrap().trim(), it.next().unwrap().trim())
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Pattern(u16);

impl Pattern {
    fn num_bits(&self) -> u32 {
        (0..7)
            .map(|x| if (self.0 & (1 << x)) > 0 { 1 } else { 0 })
            .sum()
    }
}

impl TryFrom<char> for Pattern {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Pattern(0b1000000)),
            'b' => Ok(Pattern(0b0100000)),
            'c' => Ok(Pattern(0b0010000)),
            'd' => Ok(Pattern(0b0001000)),
            'e' => Ok(Pattern(0b0000100)),
            'f' => Ok(Pattern(0b0000010)),
            'g' => Ok(Pattern(0b0000001)),
            _ => Err("Only a through g"),
        }
    }
}
impl TryFrom<&str> for Pattern {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .chars()
            .try_fold(Pattern(0), |pattern, c| Ok(pattern | Pattern::try_from(c)?))
    }
}

impl TryInto<u32> for Pattern {
    type Error = &'static str;

    fn try_into(self) -> Result<u32, Self::Error> {
        match self.0 {
            0b111011 => Ok(0),
            0b0010010 => Ok(1),
            0b1011101 => Ok(2),
            0b1011011 => Ok(3),
            0b0111010 => Ok(4),
            0b1101011 => Ok(5),
            0b1101111 => Ok(6),
            0b1010010 => Ok(7),
            0b1111111 => Ok(8),
            0b1111011 => Ok(9),
            _ => Err("Not a real pattern"),
        }
    }
}

impl BitAnd for Pattern {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Pattern(self.0 & rhs.0)
    }
}

impl BitOr for Pattern {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Pattern(self.0 | rhs.0)
    }
}

fn decode_entry(entry: &(&str, &str)) -> u32 {
    let (input, output) = entry;
    let inputs: Vec<&str> = input.split_whitespace().collect();

    let one_pattern = Pattern::try_from(
        inputs
            .iter()
            .find_map(|&x| if x.len() == 2 { Some(x) } else { None })
            .expect("Pattern for 1"),
    )
    .expect("Convert pattern for 1");

    let four_pattern = Pattern::try_from(
        inputs
            .iter()
            .find_map(|&x| if x.len() == 4 { Some(x) } else { None })
            .expect("Pattern for 4"),
    )
    .expect("Convert pattern for 4");

    let seven_pattern = Pattern::try_from(
        inputs
            .iter()
            .find_map(|&x| if x.len() == 3 { Some(x) } else { None })
            .expect("Pattern for 7"),
    )
    .expect("Convert pattern for 7");

    let mut patterns = [Pattern(0); 10];
    patterns[1] = one_pattern;
    patterns[4] = four_pattern;
    patterns[7] = seven_pattern;

    for input in inputs {
        let input_pattern = Pattern::try_from(input).expect("invalid pattern");

        match input_pattern.num_bits() {
            7 => patterns[8] = input_pattern,
            6 => {
                let num_with_1 = (one_pattern & input_pattern).num_bits();
                let num_with_4 = (four_pattern & input_pattern).num_bits();

                if num_with_1 == 1 {
                    patterns[6] = input_pattern;
                } else if num_with_4 == 3 {
                    patterns[0] = input_pattern;
                } else {
                    patterns[9] = input_pattern;
                }
            }
            5 => {
                let num_with_1 = (one_pattern & input_pattern).num_bits();
                let num_with_4_7 = ((four_pattern | seven_pattern) & input_pattern).num_bits();

                if num_with_1 == 2 {
                    patterns[3] = input_pattern;
                } else if num_with_4_7 == 3 {
                    patterns[2] = input_pattern;
                } else {
                    patterns[5] = input_pattern;
                }
            }
            _ => (),
        };
    }

    output
        .trim()
        .split_whitespace()
        .filter_map(|x| Pattern::try_from(x).ok())
        .map(|p| {
            patterns
                .iter()
                .position(|&p_candidate| p == p_candidate)
                .unwrap()
        })
        .reduce(|total, num| (total * 10) + num)
        .unwrap() as u32
}

fn count_unique_numbers(lines: &Vec<String>) -> usize {
    let unique_number_signals: [u32; 4] = [2, 3, 4, 7];
    lines
        .iter()
        .map(|line| {
            let (_, output) = split_input(line);
            output
        })
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.len())
                .filter(|&len| unique_number_signals.contains(&(len as u32)))
                .count()
        })
        .sum::<usize>()
}

pub fn part1() {
    let input_file = BufReader::new(File::open("input/day8.txt").unwrap());

    let full_input: Vec<String> = input_file
        .lines()
        .map(|result| result.expect("valid line"))
        .collect();

    let count = count_unique_numbers(&full_input);

    println!("Number of unique numbers {}", count);
}
pub fn part2() {
    let input_file = BufReader::new(File::open("input/day8.txt").unwrap());
    let lines: Vec<String> = input_file
        .lines()
        .map(|result| result.expect("valid line"))
        .collect();

    let sum = lines
        .iter()
        .map(|line| split_input(&line))
        .map(|entry| decode_entry(&entry))
        .sum::<u32>();

    println!("Sum of decoded output: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_input_test() {
        let s = "bgeacd dbfag bcadegf agdce dgfbce bgc bdgca aedcgf bc abec | gcdfbe cbea bc gbc";

        let (first, second) = split_input(s);

        assert_eq!(
            first,
            "bgeacd dbfag bcadegf agdce dgfbce bgc bdgca aedcgf bc abec"
        );

        assert_eq!(second, "gcdfbe cbea bc gbc");
    }

    #[test]
    fn sample_data_part1() {
        let test_data_part1 = vec![
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
"edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc    ",
"fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg         ",
"fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb   ",
"aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea   ",
"fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb  ",
"dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe  ",
"bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef    ",
"egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb       ",
"gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce      ",
]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let c = count_unique_numbers(&test_data_part1);
        assert_eq!(c, 26);
    }

    #[test]
    fn decode_entry_test() {
        let test_data =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

        let entry = split_input(test_data);

        let output = decode_entry(&entry);

        assert_eq!(output, 5353);
    }

    #[test]
    fn sample_data_part2() {
        let lines = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc    ",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg         ",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb   ",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea   ",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb  ",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe  ",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef    ",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb       ",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce      ",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let sum = lines
            .iter()
            .map(|line| split_input(&line))
            .map(|entry| decode_entry(&entry))
            .sum::<u32>();

        assert_eq!(sum, 61229);
    }

    #[test]
    fn num_bits_test() {
        let x = Pattern(0b1001);
        assert_eq!(x.num_bits(), 2);
    }
}
