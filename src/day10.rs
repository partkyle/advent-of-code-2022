#[cfg(test)]
pub mod day10 {
    use std::{collections::HashMap, str::FromStr};

    use itertools::Itertools;

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

    fn run_instructions(instructions: Vec<Instruction>) -> HashMap<i32, i32> {
        let mut x = 1;
        let mut cycle = 1;
        let mut cycles = HashMap::new();
        cycles.insert(cycle, x);

        for inst in instructions {
            cycle += inst.cycles();

            match inst {
                Instruction::Noop => {}
                Instruction::Addx(cdr) => {
                    x += cdr;
                    cycles.insert(cycle, x);
                }
            };
        }

        cycles
    }

    fn read_instructions(text: String) -> Result<Vec<Instruction>, Box<(dyn std::error::Error)>> {
        text.lines().map(|line| line.parse()).collect()
    }

    fn get_cycle_value_at(cycles: &HashMap<i32, i32>, i: i32) -> i32 {
        let key = cycles.keys().filter(|&&k| k <= i).sorted().last().unwrap();
        *cycles.get(key).unwrap()
    }

    pub fn part1(text: String) -> Result<i32, Box<dyn std::error::Error>> {
        let instructions = read_instructions(text)?;

        let cycles = run_instructions(instructions);

        let important_values = [20, 60, 100, 140, 180, 220];
        let result = important_values
            .iter()
            .map(|&i| i * get_cycle_value_at(&cycles, i))
            .sum();

        Ok(result)
    }

    fn cycle_to_scanline(i: i32) -> i32 {
        (i - 1) % 40
    }

    pub fn part2(text: String) -> Result<String, Box<dyn std::error::Error>> {
        let scan_lines: Vec<Vec<i32>> = (0..6)
            .map(|i| {
                let start = i * 40;
                (start + 1..=start + 40).collect()
            })
            .collect();

        let instructions = read_instructions(text)?;
        let cycles = run_instructions(instructions);

        let result: Vec<String> = scan_lines
            .iter()
            .map(|line| {
                line.iter()
                    .map(|&i| {
                        let x = get_cycle_value_at(&cycles, i);

                        let scanvalue = cycle_to_scanline(i);
                        if x - 1 <= scanvalue && scanvalue <= x + 1 {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(result.join("\n"))
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
