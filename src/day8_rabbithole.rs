use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

fn split_input(s: &str) -> (&str, &str) {
    let mut it = s.split('|');
    (it.next().unwrap().trim(), it.next().unwrap().trim())
}

fn is_solved(map: &HashMap<char, Vec<char>>) -> bool {
    for (_, y) in map {
        if y.len() > 1 {
            return false;
        }
    }
    true
}

fn possible_arrangements<'a>(
    state: &HashMap<char, Vec<char>>,
    input: &str,
    all_arrangements: &Vec<&'a str>,
) -> Vec<&'a str> {
    let length = input.len();
    let filtered = all_arrangements
        .iter()
        .filter(|&&arrangement| arrangement.len() == length)
        .filter(|&&arrangement| {
            input.chars().all(|c| {
                arrangement
                    .chars()
                    .any(|c_arrangement| state[&c].contains(&c_arrangement))
            })
        })
        .map(|&s| s)
        .collect();

    filtered
}

fn remove_not_possible_letters(
    state: &mut HashMap<char, Vec<char>>,
    input: &str,
    possible_arrangements: &Vec<&str>,
) {
    for c_in in input.chars() {
        state.entry(c_in).and_modify(|mappings| {
            *mappings = mappings
                .iter()
                .filter(|&&c| {
                    possible_arrangements
                        .iter()
                        .any(|arrangement| arrangement.contains(c))
                })
                .map(|&c| c)
                .collect();
        });
    }
}

fn remove_possibilities(state: &mut HashMap<char, Vec<char>>, letters: &str, except_from: &str) {
    let keys = state
        .keys()
        .filter_map(|&key| {
            if !except_from.contains(key) {
                Some(key)
            } else {
                None
            }
        })
        .collect::<Vec<char>>();
    keys.iter().for_each(|&key| {
        let key = key.clone();
        state
            .entry(key)
            .and_modify(|chars| chars.retain(|c| !letters.contains(*c)));
    });
}

fn decode_entry(entry: &(&str, &str)) -> u32 {
    let (input, output) = entry;

    let mut inputs: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    let mut possibilities = HashMap::from([
        ('a', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        ('b', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        ('c', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        ('d', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        ('e', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        ('f', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        ('g', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
    ]);

    let mut arrangements = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];

    let mut inputs_to_remove = Vec::new();
    while !is_solved(&possibilities) {
        let mut num_removed = 0;
        for i in &inputs_to_remove {
            inputs.remove(*i - num_removed as usize);
            num_removed = num_removed + 1;
        }
        inputs_to_remove.clear();

        inputs.sort_by(|x, y| x.len().cmp(&y.len()));
        for (i, input) in inputs.iter().enumerate() {
            let possible_arrangements = possible_arrangements(&possibilities, input, &arrangements);
            remove_not_possible_letters(&mut possibilities, input, &possible_arrangements);

            if possible_arrangements.len() == 1 {
                let arrangement = possible_arrangements[0];
                let index = arrangements.iter().position(|&s| s == arrangement).unwrap();
                arrangements.swap_remove(index);

                remove_possibilities(&mut possibilities, possible_arrangements[0], input);
                inputs_to_remove.push(i);
            }
        }
    }

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

    //#[test]
    //fn sample_data_part2() {
    //    let lines = vec![
    //        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
    //        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc    ",
    //        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg         ",
    //        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb   ",
    //        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea   ",
    //        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb  ",
    //        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe  ",
    //        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef    ",
    //        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb       ",
    //        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce      ",
    //    ]
    //    .iter()
    //    .map(|s| s.to_string())
    //    .collect::<Vec<String>>();
    //    let sum = lines
    //        .iter()
    //        .map(|line| split_input(&line))
    //        .map(|entry| decode_entry(&entry))
    //        .sum::<u32>();

    //    assert_eq!(sum, 61229);
    //}

    #[test]
    fn possible_arrangements_test() {
        let input = "abcd";
        let state = HashMap::from([
            ('a', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            ('b', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            ('c', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            ('d', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            ('e', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            ('f', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            ('g', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        ]);

        let arrangements = vec![
            "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
        ];

        let possibles = possible_arrangements(&state, input, &arrangements);

        assert_eq!(possibles.len(), 1);
        assert_eq!(possibles[0], "bcdf");

        let arrangements = vec!["abc", "bcd"];
        let input = "efg";
        let state = HashMap::from([
            ('e', vec!['a']),
            ('f', vec!['b', 'c', 'd']),
            ('g', vec!['b', 'c', 'd']),
        ]);

        let possibles = possible_arrangements(&state, input, &arrangements);
        assert_eq!(possibles.len(), 1);
        assert_eq!(possibles[0], "abc");
    }

    #[test]
    fn remove_not_possible_letters_test() {
        let input = "abcd";
        let mut state = HashMap::from([
            ('a', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            ('b', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            ('c', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            ('d', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            ('e', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            ('f', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            ('g', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        ]);

        let arrangements = vec![
            "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
        ];

        let possibles = possible_arrangements(&state, input, &arrangements);

        remove_not_possible_letters(&mut state, input, &possibles);

        let mut a_s = state[&'a'].clone();
        let mut b_s = state[&'b'].clone();
        let mut c_s = state[&'c'].clone();
        let mut d_s = state[&'d'].clone();

        a_s.sort();
        b_s.sort();
        c_s.sort();
        d_s.sort();

        assert_eq!(a_s, vec!['b', 'c', 'd', 'f']);
        assert_eq!(b_s, vec!['b', 'c', 'd', 'f']);
        assert_eq!(c_s, vec!['b', 'c', 'd', 'f']);
        assert_eq!(d_s, vec!['b', 'c', 'd', 'f']);

        let arrangements = vec!["abc", "bcd"];
        let input = "efg";
        let mut state = HashMap::from([
            ('e', vec!['a']),
            ('f', vec!['b', 'c', 'd']),
            ('g', vec!['b', 'c', 'd']),
        ]);

        let possibles = possible_arrangements(&state, input, &arrangements);

        remove_not_possible_letters(&mut state, input, &possibles);

        let mut e_s = state[&'e'].clone();
        let mut f_s = state[&'f'].clone();
        let mut g_s = state[&'g'].clone();

        e_s.sort();
        f_s.sort();
        g_s.sort();

        assert_eq!(e_s, vec!['a']);
        assert_eq!(f_s, vec!['b', 'c']);
        assert_eq!(g_s, vec!['b', 'c']);
    }

    #[test]
    fn remove_possibilities_test() {
        let mut state = HashMap::from([
            ('e', vec!['a']),
            ('f', vec!['b', 'c', 'd']),
            ('g', vec!['b', 'c', 'd']),
        ]);

        remove_possibilities(&mut state, "cd", "g");

        assert_eq!(state[&'e'], vec!['a']);
        assert_eq!(state[&'f'], vec!['b']);
        assert_eq!(state[&'g'], vec!['b', 'c', 'd']);
    }
}
