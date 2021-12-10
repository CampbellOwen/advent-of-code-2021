use std::{convert::TryFrom, fs::File, io::BufRead, io::BufReader};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Chunk {
    Paren,
    Bracket,
    Brace,
    Angle,
}

impl TryFrom<char> for Chunk {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Chunk::Paren),
            '[' => Ok(Chunk::Bracket),
            '{' => Ok(Chunk::Brace),
            '<' => Ok(Chunk::Angle),
            _ => Err("Not a valid Chunk delimiter"),
        }
    }
}

impl Chunk {
    fn parse_end(c: char) -> Result<Chunk, &'static str> {
        match c {
            ')' => Ok(Chunk::Paren),
            ']' => Ok(Chunk::Bracket),
            '}' => Ok(Chunk::Brace),
            '>' => Ok(Chunk::Angle),
            _ => Err("Not a valid Chunk ending"),
        }
    }
}

fn error_points(chunk: Chunk) -> u32 {
    match chunk {
        Chunk::Paren => 3,
        Chunk::Bracket => 57,
        Chunk::Brace => 1197,
        Chunk::Angle => 25137,
    }
}

fn error_points_2(chunk: Chunk) -> usize {
    match chunk {
        Chunk::Paren => 1,
        Chunk::Bracket => 2,
        Chunk::Brace => 3,
        Chunk::Angle => 4,
    }
}

#[derive(Debug, PartialEq)]
enum ParseError {
    Incomplete(Vec<Chunk>),
    Corrupt(Chunk),
}

fn parse_line(line: &str) -> Result<(), ParseError> {
    let mut stack = Vec::new();
    for c in line.trim().chars() {
        if let Ok(chunk) = Chunk::try_from(c) {
            stack.push(chunk)
        } else if let Ok(ending) = Chunk::parse_end(c) {
            if let Some(start) = stack.last() {
                if start == &ending {
                    stack.pop();
                } else {
                    return Err(ParseError::Corrupt(ending));
                }
            }
        }
    }

    if stack.is_empty() {
        Ok(())
    } else {
        Err(ParseError::Incomplete(stack))
    }
}

pub fn part1() {
    let input = BufReader::new(File::open("input/day10.txt").unwrap());
    let error_points: u32 = input
        .lines()
        .filter_map(|line| {
            let line = line.expect("Invalid line");
            match parse_line(&line) {
                Ok(_) => None,
                Err(err) => match err {
                    ParseError::Incomplete(_) => None,
                    ParseError::Corrupt(chunk) => Some(error_points(chunk)),
                },
            }
        })
        .sum();

    println!("Total error points: {}", error_points);
}

fn score_incomplete(remaining: Vec<Chunk>) -> usize {
    remaining
        .iter()
        .rev()
        .fold(0, |total, &chunk| (total * 5) + error_points_2(chunk))
}

pub fn part2() {
    let input = BufReader::new(File::open("input/day10.txt").unwrap());
    let mut error_scores: Vec<usize> = input
        .lines()
        .filter_map(|line| {
            let line = line.expect("Invalid line");
            match parse_line(&line) {
                Ok(_) => None,
                Err(err) => match err {
                    ParseError::Incomplete(unmatched) => Some(score_incomplete(unmatched)),
                    ParseError::Corrupt(_) => None,
                },
            }
        })
        .collect();

    error_scores.sort();
    let mid = error_scores[error_scores.len() / 2];

    println!("Middle error score: {}", mid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_chunk_test() {
        let chunk = Chunk::try_from('(').unwrap();
        assert_eq!(chunk, Chunk::Paren);

        let chunk = Chunk::try_from('[').unwrap();
        assert_eq!(chunk, Chunk::Bracket);
    }

    #[test]
    fn parse_line_test() {
        let line = "[({(<(())[]>[[{[]{<()<>>";
        let result = parse_line(line);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(
            error,
            ParseError::Incomplete(vec![
                Chunk::Bracket,
                Chunk::Paren,
                Chunk::Brace,
                Chunk::Paren,
                Chunk::Bracket,
                Chunk::Bracket,
                Chunk::Brace,
                Chunk::Brace,
            ])
        );

        let line = "{([(<{}[<>[]}>{[]{[(<()>";
        let result = parse_line(line);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error, ParseError::Corrupt(Chunk::Brace));

        let line = "[[<[([]))<([[{}[[()]]]";
        let result = parse_line(line);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error, ParseError::Corrupt(Chunk::Paren));

        let line = "[{[{({}]{}}([{[{{{}}([]";
        let result = parse_line(line);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error, ParseError::Corrupt(Chunk::Bracket));

        let line = "<{([([[(<>()){}]>(<<{{";
        let result = parse_line(line);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error, ParseError::Corrupt(Chunk::Angle));

        let line = "<{([{{}}[<[[[<>{}]]]>[]]";
        let result = parse_line(line);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(
            error,
            ParseError::Incomplete(vec![
                Chunk::Angle,
                Chunk::Brace,
                Chunk::Paren,
                Chunk::Bracket
            ])
        );
    }

    #[test]
    fn incomplete_error_test() {
        let line = "<{([{{}}[<[[[<>{}]]]>[]]";
        let result = parse_line(line);
        assert!(result.is_err());

        let error = result.unwrap_err();
        if let ParseError::Incomplete(remaining) = error {
            assert_eq!(score_incomplete(remaining), 294);
        } else {
            assert!(false);
        }
    }
}
