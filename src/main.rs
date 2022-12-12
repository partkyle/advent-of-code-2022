use core::panic;
use itertools::Itertools;
use reqwest::blocking::Response;
use std::{
    env,
    fmt::{Debug, Display},
    fs,
    ops::Deref,
    path::PathBuf,
};

const SESSION_TOKEN_PATH: &str = ".advent";

fn get_token() -> Result<String, Box<dyn std::error::Error>> {
    let p = PathBuf::new()
        .join(env::var("HOME")?)
        .join(SESSION_TOKEN_PATH);

    fs::read_to_string(p)
        .map(|s| s.trim().to_string())
        .map_err(|e| e.into())
}

fn get_daily_args(day: isize) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    let session_header = format!("session={}", get_token()?);
    let s = &session_header[..];

    client
        .get(format!("https://adventofcode.com/2022/day/{}/input", day))
        .header("cookie", s)
        .send()
        .map_err(|e| e.into())
}

#[derive(Debug)]
struct Elf {
    calories: Vec<isize>,
}

impl Elf {
    fn new() -> Elf {
        return Elf { calories: vec![] };
    }

    fn add_calorie(&mut self, calorie: isize) {
        self.calories.push(calorie);
    }

    fn total(&self) -> isize {
        self.calories.iter().sum()
    }
}

fn day_01_01(response: Response) -> Result<isize, Box<dyn std::error::Error>> {
    let mut elves: Vec<Elf> = vec![];
    let mut current_elf = Elf::new();

    let text = response.text()?;

    for line in text.lines() {
        if line == "" {
            elves.push(current_elf);
            current_elf = Elf::new();
            continue;
        }

        let calorie: Result<isize, _> = line.parse();
        current_elf.add_calorie(calorie?);
    }

    let result = elves.iter().map(|e| e.total()).max();

    result.ok_or_else(|| "no sum found".into())
}

fn day_01_02(response: Response) -> Result<isize, Box<dyn std::error::Error>> {
    let mut elves: Vec<Elf> = vec![];
    let mut current_elf = Elf::new();

    let text = response.text()?;

    for line in text.lines() {
        if line == "" {
            elves.push(current_elf);
            current_elf = Elf::new();
            continue;
        }

        let calorie: Result<isize, _> = line.parse();
        current_elf.add_calorie(calorie?);
    }

    let result: isize = elves
        .iter()
        .map(|e| e.total())
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum();

    Ok(result)
}

fn day_02_01(response: Response) -> Result<isize, Box<dyn std::error::Error>> {
    let text = response.text()?;

    let matches: Result<Vec<rps_line>, _> = text.lines().map(|line| line.try_into()).collect();

    Ok(matches?.iter().map(|m| m.score()).sum())
}

fn day_02_02(response: Response) -> Result<isize, Box<dyn std::error::Error>> {
    let text = response.text()?;

    let matches: Result<Vec<rps_line_2>, _> = text.lines().map(|line| line.try_into()).collect();

    Ok(matches?.iter().map(|m| m.score()).sum())
}

#[derive(Debug)]
struct rps_line {
    theirs: RPS,
    mine: RPS,
}

impl rps_line {
    fn score(&self) -> isize {
        self.mine.point_value() + self.mine.score(self.theirs)
    }
}

impl TryFrom<&str> for rps_line {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("value does not have 2 parts".to_string());
        }

        let result = rps_line {
            theirs: parts[0].try_into()?,
            mine: parts[1].try_into()?,
        };

        Ok(result)
    }
}

#[derive(Debug)]
struct rps_line_2 {
    theirs: RPS,
    mine: Outcome,
}

impl rps_line_2 {
    fn score(&self) -> isize {
        let my_guess = self.mine.get_rps_based_on_other_player(self.theirs);
        my_guess.point_value() + my_guess.score(self.theirs)
    }
}

impl TryFrom<&str> for rps_line_2 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("value does not have 2 parts".to_string());
        }

        let result = rps_line_2 {
            theirs: parts[0].try_into()?,
            mine: parts[1].try_into()?,
        };

        Ok(result)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn beats(self, other: RPS) -> bool {
        match other {
            RPS::Rock => self == RPS::Paper,
            RPS::Paper => self == RPS::Scissors,
            RPS::Scissors => self == RPS::Rock,
        }
    }

    fn point_value(self) -> isize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn score(self, other: RPS) -> isize {
        if self == other {
            3
        } else if self.beats(other) {
            6
        } else {
            0
        }
    }
}

impl TryFrom<&str> for RPS {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(RPS::Rock),
            "X" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "Y" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissors),
            "Z" => Ok(RPS::Scissors),
            _ => Err(format!("invalid value {}", value)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn get_rps_based_on_other_player(self, other: RPS) -> RPS {
        match self {
            Outcome::Lose => match other {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            },
            Outcome::Draw => other,
            Outcome::Win => match other {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            },
        }
    }
}

impl TryFrom<&str> for Outcome {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(format!("invalid value {}", value)),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = get_daily_args(2)?;

    let result = day_02_02(response);

    match result {
        Ok(val) => {
            println!("{}", val);
            Ok(())
        }
        Err(err) => return Err(err),
    }
}

#[cfg(test)]
mod test {
    use reqwest::blocking::Response;

    use crate::{day_01_01, day_01_02, day_02_01, day_02_02, get_daily_args};

    fn run_day<T: std::fmt::Display>(
        day: isize,
        f: fn(Response) -> Result<T, Box<dyn std::error::Error>>,
    ) {
        let response = get_daily_args(day).unwrap();
        let value = f(response);
        println!("{}", value.unwrap());
    }

    #[test]
    fn test_day_01_01() {
        run_day(1, day_01_01);
    }

    #[test]
    fn test_day_01_02() {
        run_day(1, day_01_02);
    }

    #[test]
    fn test_day_02_01() {
        run_day(2, day_02_01);
    }

    #[test]
    fn test_day_02_02() {
        run_day(2, day_02_02);
    }
}
