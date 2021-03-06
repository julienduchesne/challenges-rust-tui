use std::collections::HashSet;

use anyhow::Result;

use crate::{
    groups::challenge_config::{ChallengeConfig, ChallengeError},
    utils::InputUtils,
};
pub struct Day16 {}
struct Rule {
    name: String,
    conditions: Vec<(usize, usize)>,
    possible_positions: HashSet<usize>,
}

impl Rule {
    fn parse(rule: &str, positions: usize) -> Result<Self> {
        let name = rule
            .split(":")
            .nth(0)
            .ok_or(ChallengeError::new("Failed to get rule name"))?;
        let ranges_str = rule
            .split(":")
            .nth(1)
            .ok_or(ChallengeError::new("Failed to get rule ranges"))?;
        let mut ranges: Vec<(usize, usize)> = vec![];
        for range in ranges_str.split_whitespace() {
            if !range.contains("-") {
                continue;
            }
            let first = range
                .split("-")
                .nth(0)
                .unwrap_or("0")
                .trim()
                .parse::<usize>()?;
            let second = range
                .split("-")
                .nth(1)
                .unwrap_or("0")
                .trim()
                .parse::<usize>()?;
            ranges.push((first, second));
        }
        return Ok(Self {
            name: name.to_owned(),
            conditions: ranges,
            possible_positions: (0..positions).collect(),
        });
    }
}

impl ChallengeConfig for Day16 {
    fn title(&self) -> &str {
        return "Day 16: Ticket Translation";
    }

    fn solve(&self, input: &str) -> Result<String> {
        let groups = input
            .replace("nearby tickets:\n", "")
            .replace("your ticket:\n", "")
            .split_sections();
        if groups.len() != 3 {
            return Err(ChallengeError::new("Expected 3 groups").into());
        }

        let rule_lines = groups[0].split("\n");
        let line_count = rule_lines.clone().count();
        let mut rules: Vec<Rule> = rule_lines
            .map(|r| Rule::parse(r.trim(), line_count))
            .collect::<Result<_, _>>()?;

        let own_ticket: Vec<usize> = groups[1]
            .split(",")
            .map(|x| x.trim().parse::<usize>())
            .collect::<Result<_, _>>()?;

        let mut other_tickets: Vec<Vec<usize>> = groups[2]
            .split_whitespace()
            .map(|x| x.split(",").map(|x| x.trim().parse::<usize>()).collect())
            .collect::<Result<_, _>>()?;

        // Part one: Find and remove bad tickets
        let mut part_one: usize = 0;
        other_tickets.retain(|ticket| {
            let mut ticket_ok = true;
            for number in ticket {
                let mut ok = false;
                for rule in &rules {
                    if rule
                        .conditions
                        .iter()
                        .any(|x| x.0 <= *number && x.1 >= *number)
                    {
                        ok = true;
                        break;
                    }
                }
                if !ok {
                    ticket_ok = false;
                    part_one += number;
                }
            }
            return ticket_ok;
        });

        // Part two: Match fields with own ticket. Loop until all rules have only one possible field left
        let mut remaining_columns: HashSet<usize> = (0..rules.len()).collect();
        loop {
            for rule in rules.iter_mut().filter(|r| r.possible_positions.len() > 1) {
                let conditions = rule.conditions.clone();
                rule.possible_positions.retain(|pos| {
                    if !remaining_columns.contains(pos) {
                        return false;
                    }

                    for ticket in &other_tickets {
                        let number = ticket.get(*pos).unwrap();
                        if !conditions.iter().any(|x| x.0 <= *number && x.1 >= *number) {
                            return false;
                        }
                    }
                    return true;
                });
                if rule.possible_positions.len() == 1 {
                    remaining_columns.remove(rule.possible_positions.iter().next().unwrap());
                }
            }

            if rules.iter().all(|r| r.possible_positions.len() == 1) {
                break;
            }
        }

        // Multiply the "departure" fields together
        let part_two: usize = rules
            .iter()
            .filter(|r| r.name.starts_with("departure"))
            .map(|r| own_ticket.get(*r.possible_positions.iter().next().unwrap()))
            .product::<Option<usize>>()
            .unwrap();

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
            "departure class: 1-3 or 5-7
            departure row: 6-11 or 33-44
            seat: 13-40 or 45-50

            your ticket:
            7,1,14

            nearby tickets:
            7,3,47
            40,4,50
            55,2,20
            38,6,12",
            "Part 1: 71\nPart 2: 7"
        )
    )]
    fn solve(input: &str, expected: &str) {
        let day = Day16 {};
        assert_eq!(day.solve(input).unwrap(), expected);
    }
}
