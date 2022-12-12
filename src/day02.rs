#[cfg(test)]
pub mod day02 {
    use reqwest::blocking::Response;
    use std::fmt::Debug;

    #[derive(Debug)]
    struct RPSLine {
        theirs: RPS,
        mine: RPS,
    }

    impl RPSLine {
        fn score(&self) -> isize {
            self.mine.point_value() + self.mine.score(self.theirs)
        }
    }

    impl TryFrom<&str> for RPSLine {
        type Error = String;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let parts: Vec<&str> = value.split_whitespace().collect();
            if parts.len() != 2 {
                return Err("value does not have 2 parts".to_string());
            }

            let result = RPSLine {
                theirs: parts[0].try_into()?,
                mine: parts[1].try_into()?,
            };

            Ok(result)
        }
    }

    #[derive(Debug)]
    struct RPSLine2 {
        theirs: RPS,
        mine: Outcome,
    }

    impl RPSLine2 {
        fn score(&self) -> isize {
            let my_guess = self.mine.get_rps_based_on_other_player(self.theirs);
            my_guess.point_value() + my_guess.score(self.theirs)
        }
    }

    impl TryFrom<&str> for RPSLine2 {
        type Error = String;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let parts: Vec<&str> = value.split_whitespace().collect();
            if parts.len() != 2 {
                return Err("value does not have 2 parts".to_string());
            }

            let result = RPSLine2 {
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

    pub fn part1(response: Response) -> Result<isize, Box<dyn std::error::Error>> {
        let text = response.text()?;

        let matches: Result<Vec<RPSLine>, _> = text.lines().map(|line| line.try_into()).collect();

        Ok(matches?.iter().map(|m| m.score()).sum())
    }

    pub fn part2(response: Response) -> Result<isize, Box<dyn std::error::Error>> {
        let text = response.text()?;

        let matches: Result<Vec<RPSLine2>, _> = text.lines().map(|line| line.try_into()).collect();

        Ok(matches?.iter().map(|m| m.score()).sum())
    }
}

#[cfg(test)]
mod test {
    use crate::helper::helper::run_day;

    use super::day02::{self};

    #[test]
    fn part1() {
        run_day(2, day02::part1);
    }

    #[test]
    fn part2() {
        run_day(2, day02::part2);
    }
}
