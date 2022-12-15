#[cfg(test)]
pub mod template {
    pub fn part1(_text: String) -> Result<isize, Box<dyn std::error::Error>> {
        todo!("need to do it");
    }

    pub fn part2(_text: String) -> Result<isize, Box<dyn std::error::Error>> {
        todo!("not doing it");
    }

    #[cfg(test)]
    mod test {
        use crate::helper::helper::run_day;

        use super::*;

        const DAY: usize = 1;

        #[test]
        fn test_part1() {
            run_day(DAY, part1);
        }

        #[test]
        fn test_part2() {
            run_day(DAY, part2);
        }
    }
}
