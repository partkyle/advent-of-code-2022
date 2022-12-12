#[cfg(test)]
pub mod template {
    use reqwest::blocking::Response;

    pub fn part1(_response: Response) -> Result<isize, Box<dyn std::error::Error>> {
        todo!("need to do it");
    }

    pub fn part2(_response: Response) -> Result<isize, Box<dyn std::error::Error>> {
        todo!("not doing it");
    }
}

#[cfg(test)]
mod test {
    use crate::helper::helper::run_day;

    use super::template;

    const DAY: isize = 1;

    #[test]
    fn part1() {
        run_day(DAY, template::part1);
    }

    #[test]
    fn part2() {
        run_day(DAY, template::part2);
    }
}
