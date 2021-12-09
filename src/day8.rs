use std::{fs::File, io::BufRead, io::BufReader};

fn split_input(s: &str) -> (&str, &str) {
    let mut it = s.split('|');
    (it.next().unwrap().trim(), it.next().unwrap().trim())
}

fn decode_entry(entry: &(&str, &str)) -> u32 {
    let (input, output) = entry;

    0
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
pub fn part2() {}

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
}
