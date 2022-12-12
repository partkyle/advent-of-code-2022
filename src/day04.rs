#[cfg(test)]
pub mod day04 {
    use std::collections::{hash_map::RandomState, HashSet};

    use reqwest::blocking::Response;

    pub struct Section {
        start: isize,
        end: isize,
    }

    impl Section {
        pub fn new(start: isize, end: isize) -> Section {
            Section {
                start: start,
                end: end,
            }
        }

        fn fully_contains(&self, b: &Section) -> bool {
            self.start <= b.start && self.end >= b.end
        }

        pub fn to_seq(&self) -> Vec<isize> {
            (self.start..=self.end).collect()
        }
    }

    fn parse_section(s: &str) -> Section {
        let mut parts = s.split("-");

        Section {
            start: parts.next().unwrap().parse().unwrap(),
            end: parts.next().unwrap().parse().unwrap(),
        }
    }

    struct Row {
        a: Section,
        b: Section,
    }

    fn parse_row(s: &str) -> Row {
        let parts: Vec<&str> = s.split(",").collect();

        Row {
            a: parse_section(parts[0]),
            b: parse_section(parts[1]),
        }
    }

    pub fn part1(response: Response) -> Result<usize, Box<dyn std::error::Error>> {
        let text = response.text()?;
        let lines = text.lines();

        let result = lines
            .map(parse_row)
            .filter(|row| {
                let a = &row.a;
                let b = &row.b;

                a.fully_contains(&b) || b.fully_contains(&a)
            })
            .count();

        Ok(result)
    }

    pub fn part2(response: Response) -> Result<usize, Box<dyn std::error::Error>> {
        let text = response.text()?;
        let lines = text.lines();

        let rows = lines
            .map(parse_row)
            .map(|row| {
                let row_a = row.a.to_seq();
                let row_b = row.b.to_seq();
                let a: HashSet<&isize, RandomState> = HashSet::from_iter(row_a.iter());
                let b: HashSet<&isize, RandomState> = HashSet::from_iter(row_b.iter());

                a.intersection(&b).count()
            })
            .filter(|&count| count > 0)
            .count();

        Ok(rows)
    }
}

#[cfg(test)]
mod test {
    use crate::helper::helper::run_day;

    use super::day04;

    const DAY: isize = 4;

    #[test]
    fn row_overlap() {
        let a = day04::Section::new(1, 5);
        assert_eq!(a.to_seq(), vec!(1,2,3,4,5));
    }

    #[test]
    fn part1() {
        run_day(DAY, day04::part1);
    }

    #[test]
    fn part2() {
        run_day(DAY, day04::part2);
    }
}
