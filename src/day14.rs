use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

type Rule = ((char, char), char);
type RuleSet = HashMap<(char, char), char>;

fn run_rules(
    input: &HashMap<(char, char), usize>,
    rules: &RuleSet,
    num_letters: &mut HashMap<char, usize>,
) -> HashMap<(char, char), usize> {
    let mut output = HashMap::new();
    for (&(a, b), &count) in input {
        if let Some(&rule_out) = rules.get(&(a, b)) {
            let entry = output.entry((a, rule_out)).or_insert(0);
            *entry += count;

            let entry = output.entry((rule_out, b)).or_insert(0);
            *entry += count;

            (*num_letters.entry(rule_out).or_insert(0)) += count;
        } else {
            panic!();
        }
    }

    output
}

fn parse_initial(s: &str) -> HashMap<(char, char), usize> {
    let mut output = HashMap::new();
    for i in 0..(s.len() - 1) {
        let mut chars = s[i..=(i + 1)].chars();
        let first = chars.next().expect("At least 1 char");
        let second = chars.next().expect("At least 2 char");
        debug_assert!(chars.next().is_none());

        let entry = output.entry((first, second)).or_insert(0_usize);
        *entry += 1;
    }

    output
}

fn parse_rule(s: &str) -> Rule {
    let (input, output) = s.split_once("->").expect("Rule should be well-formed");

    let input = input.trim();
    let output = output.trim();

    let mut in_chars = input.chars();
    let first = in_chars.next().expect("Should be enough characters");
    let second = in_chars.next().expect("Should be enough characters");
    debug_assert!(in_chars.next().is_none());

    let mut output_chars = output.chars();
    let new_char = output_chars.next().expect("Should be enough chars");
    debug_assert!(output_chars.next().is_none());

    ((first, second), new_char)
}

fn parse_rules(s: &str) -> RuleSet {
    let iter = s.lines().map(parse_rule);

    HashMap::from_iter(iter)
}

pub fn part1() {
    let mut input_buf =
        BufReader::new(File::open("input/day14.txt").expect("Should have input file"));

    let mut input = String::new();
    input_buf
        .read_to_string(&mut input)
        .expect("Should read file fine");

    let (initial_str, rules) = input.split_once("\n\n").expect("Should have an empty line");

    let initial_state = parse_initial(initial_str.trim());

    let rules = parse_rules(rules.trim());

    let mut char_count = HashMap::new();
    for c in initial_str.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    let _ = (0..10).fold(initial_state, |state, _| {
        run_rules(&state, &rules, &mut char_count)
    });

    let mut counts = char_count
        .iter()
        .map(|(&c, &count)| (c, count))
        .collect::<Vec<(char, usize)>>();

    counts.sort_unstable_by(|&(_, a), (_, b)| a.cmp(b));

    let diff = counts.last().unwrap().1 - counts[0].1;

    println!("Diff between smallest and biggest after 10 iter: {}", diff);
}
pub fn part2() {
    let mut input_buf =
        BufReader::new(File::open("input/day14.txt").expect("Should have input file"));

    let mut input = String::new();
    input_buf
        .read_to_string(&mut input)
        .expect("Should read file fine");

    let (initial_str, rules) = input.split_once("\n\n").expect("Should have an empty line");

    let initial_state = parse_initial(initial_str.trim());

    let rules = parse_rules(rules.trim());

    let mut char_count = HashMap::new();
    for c in initial_str.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    let _ = (0..40).fold(initial_state, |state, _| {
        run_rules(&state, &rules, &mut char_count)
    });

    let mut counts = char_count
        .iter()
        .map(|(&c, &count)| (c, count))
        .collect::<Vec<(char, usize)>>();

    counts.sort_unstable_by(|&(_, a), (_, b)| a.cmp(b));

    let diff = counts.last().unwrap().1 - counts[0].1;

    println!("Diff between smallest and biggest after 40 iter: {}", diff);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parse_test() {
        let r = "CH -> B";
        let rule = parse_rule(r);

        assert_eq!(rule, (('C', 'H'), 'B'));
    }

    #[test]
    fn example_1() {
        let input = parse_initial("NNCB");

        let rules = "CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        let rules = parse_rules(rules);

        let mut char_count = HashMap::from([('N', 2), ('C', 1), ('B', 1)]);

        let input = run_rules(&input, &rules, &mut char_count);
        //NCNBCHB
        assert_eq!(input[&('N', 'C')], 1);
        assert_eq!(input[&('C', 'N')], 1);
        assert_eq!(input[&('N', 'B')], 1);
        assert_eq!(input[&('B', 'C')], 1);
        assert_eq!(input[&('C', 'H')], 1);
        assert_eq!(input[&('H', 'B')], 1);

        assert_eq!(char_count[&'N'], 2);
        assert_eq!(char_count[&'C'], 2);
        assert_eq!(char_count[&'B'], 2);
        assert_eq!(char_count[&'H'], 1);

        let input = run_rules(&input, &rules, &mut char_count);
        // NBCCNBBBCBHCB
        assert_eq!(input[&('N', 'B')], 2);
        assert_eq!(input[&('B', 'C')], 2);
        assert_eq!(input[&('C', 'C')], 1);
        assert_eq!(input[&('C', 'N')], 1);
        assert_eq!(input[&('B', 'B')], 2);
        assert_eq!(input[&('C', 'B')], 2);
        assert_eq!(input[&('B', 'H')], 1);
        assert_eq!(input[&('H', 'C')], 1);

        assert_eq!(char_count[&'N'], 2);
        assert_eq!(char_count[&'C'], 4);
        assert_eq!(char_count[&'B'], 6);
        assert_eq!(char_count[&'H'], 1);

        let input = run_rules(&input, &rules, &mut char_count);
        let input = run_rules(&input, &rules, &mut char_count);
        let input = run_rules(&input, &rules, &mut char_count);
        let input = run_rules(&input, &rules, &mut char_count);
        let input = run_rules(&input, &rules, &mut char_count);
        let input = run_rules(&input, &rules, &mut char_count);
        let input = run_rules(&input, &rules, &mut char_count);
        let _ = run_rules(&input, &rules, &mut char_count);

        assert_eq!(char_count[&'N'], 865);
        assert_eq!(char_count[&'C'], 298);
        assert_eq!(char_count[&'B'], 1749);
        assert_eq!(char_count[&'H'], 161);
    }
}
