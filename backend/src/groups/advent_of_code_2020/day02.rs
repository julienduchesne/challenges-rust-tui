use anyhow::Result;
use regex::Regex;

use crate::groups::challenge_config::{ChallengeConfig, ChallengeError};

pub struct Day2 {}
#[derive(Clone)]
struct Line {
    min_chars: usize,
    max_chars: usize,
    letter: char,
    password: String,
}

impl Line {
    fn parse(line: &str) -> Result<Line, ChallengeError> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?P<min>\d+)-(?P<max>\d+)\s(?P<letter>\w{1}):\s(?P<password>\w+)")
                    .unwrap();
        }
        for caps in RE.captures_iter(line) {
            return Ok(Line {
                min_chars: (&caps["min"]).parse::<usize>().unwrap(),
                max_chars: (&caps["max"]).parse::<usize>().unwrap(),
                letter: (&caps["letter"]).to_owned().chars().nth(0).unwrap(),
                password: (&caps["password"]).to_owned(),
            });
        }
        return Err(ChallengeError::new(
            ("Could not parse line: ".to_owned() + line).as_str(),
        ));
    }
}

impl Day2 {
    fn solve_part_one(&self, lines: Vec<Line>) -> i32 {
        let mut total = 0;
        for line in lines {
            let char_count = line.password.matches(line.letter).count();
            if line.min_chars <= char_count && char_count <= line.max_chars {
                total = total + 1;
            }
        }
        return total;
    }

    fn solve_part_two(&self, lines: Vec<Line>) -> i32 {
        let mut total = 0;
        for line in lines {
            if (line.password.chars().nth(line.min_chars - 1).unwrap() == line.letter)
                ^ (line.password.chars().nth(line.max_chars - 1).unwrap() == line.letter)
            {
                total = total + 1;
            }
        }
        return total;
    }
}

impl ChallengeConfig for Day2 {
    fn title(&self) -> &str {
        return "Day 2: Password Philosophy";
    }

    fn solve(&self, input: &str) -> Result<String> {
        let lines: Vec<Line> = input
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(Line::parse)
            .collect::<Result<_, _>>()?;

        let part_one = self.solve_part_one(lines.clone());
        let part_two = self.solve_part_two(lines.clone());
        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        input,
        expected,
        case(
            "1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc",
            "Part 1: 2\nPart 2: 1"
        )
    )]
    fn solve(input: &str, expected: &str) {
        let day = Day2 {};
        assert_eq!(day.solve(input).unwrap(), expected);
    }
}
