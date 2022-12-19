#[cfg(test)]
pub mod template {
    pub fn part1(_text: String) -> Result<isize, Box<dyn std::error::Error>> {
        todo!("need to do it");
    }

    pub fn part2(_text: String) -> Result<isize, Box<dyn std::error::Error>> {
        todo!("not doing it");
    }

    mod test {
        use crate::helper::helper::{dbg_day, run_day};

        use super::*;

        const DAY: usize = 15;

        #[test]
        fn test_part1() {
            run_day(DAY, part1);
        }

        #[test]
        fn test_part2() {
            run_day(DAY, part2);
        }

        #[test]
        fn dbg_part1() {
            dbg_day(DAY, part1);
        }

        #[test]
        fn dbg_part2() {
            dbg_day(DAY, part2);
        }
    }
}
