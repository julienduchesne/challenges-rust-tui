use serde::{Deserialize, Serialize};

use super::super::{challenge_config::ChallengeConfig, group_config::GroupConfig};

const DEFAULT_PORT: i32 = 8082;

#[derive(Serialize, Deserialize)]
pub struct GoChallenge {
    id: i32,
    title: String,
    description: String,
    port: Option<i32>,
}

impl ChallengeConfig for GoChallenge {
    fn title(&self) -> &str {
        return self.title.as_str();
    }

    fn description(&self) -> &str {
        return self.description.as_str();
    }

    fn solve(&self, input: &str) -> anyhow::Result<String> {
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!(
                "http://localhost:{}/solve/{}",
                self.port.unwrap_or(DEFAULT_PORT),
                self.id
            ))
            .body(String::from(input))
            .send()?;
        return Ok(res.text()?);
    }
}

pub struct AdventOfCode2019 {
    challenges: Vec<Box<dyn ChallengeConfig>>,
}

impl AdventOfCode2019 {
    fn list_challenges(port: i32) -> anyhow::Result<Vec<GoChallenge>> {
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(format!("http://localhost:{}/list", port))
            .send()?;
        let mut v: Vec<GoChallenge> = res.json()?;
        v.iter_mut().for_each(|i| i.port = Some(port));
        return Ok(v);
    }
}

impl GroupConfig for AdventOfCode2019 {
    fn new() -> Self
    where
        Self: Sized,
    {
        let port = match std::env::var("CHALLENGES_AOC_2019_PORT") {
            Ok(p) => p.parse::<i32>().unwrap_or(DEFAULT_PORT),
            Err(_) => DEFAULT_PORT,
        };
        let challenges = match AdventOfCode2019::list_challenges(port) {
            Ok(v) => v
                .into_iter()
                .map(|e| Box::new(e) as Box<dyn ChallengeConfig>)
                .collect::<Vec<_>>(),
            Err(err) => {
                eprintln!(
                    "Got an error while listing challenges for Advent of Code 2019: {}",
                    err
                );
                vec![]
            }
        };
        return AdventOfCode2019 {
            challenges: challenges,
        };
    }

    fn name(&self) -> &str {
        return "Advent of Code 2019";
    }
    fn url(&self) -> &str {
        return "https://adventofcode.com/2019";
    }

    fn challenges(&self) -> &Vec<Box<dyn ChallengeConfig>> {
        return &self.challenges;
    }
}