#[cfg(test)]
pub mod day10 {
    use std::{collections::HashMap, str::FromStr};

    use itertools::{zip_eq, Itertools};

    #[derive(Debug)]
    enum Instruction {
        Noop,
        Addx(i32),
    }

    impl Instruction {
        fn cycles(&self) -> i32 {
            match self {
                Instruction::Noop => 1,
                Instruction::Addx(_) => 2,
            }
        }
    }

    impl FromStr for Instruction {
        type Err = Box<dyn std::error::Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.splitn(2, " ");

            let token = parts.next().ok_or("invalid token")?;

            let result = match token {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Addx(parts.next().ok_or("invalid token parts")?.parse()?),

                _ => Err(format!("invalid token: {}", token))?,
            };

            Ok(result)
        }
    }

    pub fn part1(text: String) -> Result<i32, Box<dyn std::error::Error>> {
        let instructions: Result<Vec<Instruction>, _> =
            text.lines().map(|line| line.parse()).collect();

        let mut x = 1;
        let mut cycle = 1;
        let mut cycles = HashMap::new();
        cycles.insert(cycle, x);

        for inst in instructions? {
            cycle += inst.cycles();

            match inst {
                Instruction::Noop => {}
                Instruction::Addx(cdr) => {
                    x += cdr;
                    cycles.insert(cycle, x);
                }
            };
        }

        let important_values = [20, 60, 100, 140, 180, 220];
        let result = important_values
            .iter()
            .map(|&i| {
                let key = cycles.keys().filter(|&&k| k <= i).sorted().last().unwrap();
                i * cycles.get(key).unwrap()
            })
            .sum();

        Ok(result)
    }

    pub fn part2(text: String) -> Result<isize, Box<dyn std::error::Error>> {
        todo!("not doing it");
    }
}

#[cfg(test)]
mod test {
    use crate::helper::helper::run_day;

    use super::day10;

    const DAY: usize = 10;

    #[test]
    fn part1() {
        run_day(DAY, day10::part1);
    }

    #[test]
    fn part2() {
        run_day(DAY, day10::part2);
    }
}
