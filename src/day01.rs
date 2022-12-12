pub mod day01 {
    use itertools::Itertools;
    use reqwest::blocking::Response;

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

    pub fn part1(response: Response) -> Result<isize, Box<dyn std::error::Error>> {
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

    pub fn part2(response: Response) -> Result<isize, Box<dyn std::error::Error>> {
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
}

#[cfg(test)]
mod test {
    use super::day01;
    use crate::helper::helper::run_day;

    const DAY: isize = 1;

    #[test]
    fn part1() {
        run_day(DAY, day01::part1);
    }

    #[test]
    fn part2() {
        run_day(DAY, day01::part2);
    }
}
