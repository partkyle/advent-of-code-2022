#[cfg(test)]
pub mod day06 {
    use std::collections::HashSet;

    use reqwest::blocking::Response;

    pub fn all_unique(s: &str) -> bool {
        let chars: HashSet<char> = s.chars().collect();
        chars.len() == s.len()
    }

    fn find_packet_character_count(text: String, page_size: usize) -> Option<usize> {
        for i in page_size..text.len() {
            let window = &text[i - page_size..i];

            if all_unique(window) {
                return Some(i);
            }
        }

        None
    }

    pub fn part1(response: Response) -> Result<usize, Box<dyn std::error::Error>> {
        let text = response.text()?;
        let page_size = 4;
        let answer = find_packet_character_count(text, page_size);
        answer.ok_or("no answer found".into())
    }

    pub fn part2(response: Response) -> Result<usize, Box<(dyn std::error::Error + 'static)>> {
        let text = response.text()?;
        let page_size = 14;
        let answer = find_packet_character_count(text, page_size);
        answer.ok_or("no answer found".into())
    }
}

#[cfg(test)]
mod test {
    use crate::helper::helper::run_day;

    use super::day06;

    const DAY: isize = 6;

    #[test]
    fn test_all_unique() {
        let tests = [("abcd", true), ("aabb", false), ("pmjp", false)];

        for (case, expected) in tests {
            assert_eq!(day06::all_unique(case), expected);
        }
    }

    #[test]
    fn part1() {
        run_day(DAY, day06::part1);
    }

    #[test]
    fn part2() {
        run_day(DAY, day06::part2);
    }
}
