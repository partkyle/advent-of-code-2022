#[cfg(test)]
pub mod day03 {
    use std::collections::HashSet;

    use reqwest::blocking::Response;

    pub fn priority(c: char) -> u8 {
        if 'a' <= c && c <= 'z' {
            (c as u8) - ('a' as u8) + 1
        } else if 'A' <= c && c <= 'Z' {
            (c as u8) - ('A' as u8) + 1 + 26
        } else {
            0
        }
    }

    pub fn split_into_parts(s: &str) -> [&str; 2] {
        let mid = s.len() / 2;
        [&s[..mid], &s[mid..]]
    }

    pub fn create_set_of_chars(s: &str) -> HashSet<char> {
        let mut result = HashSet::new();
        for c in s.chars() {
            result.insert(c);
        }
        result
    }

    pub fn part1(response: Response) -> Result<i32, Box<dyn std::error::Error>> {
        let result: i32 = response
            .text()?
            .lines()
            .map(split_into_parts)
            .map(|[a, b]| [create_set_of_chars(a), create_set_of_chars(b)])
            // now find the union
            .map(|[a, b]| {
                let collisions: Vec<&char> = a.intersection(&b).collect();
                collisions.iter().map(|&&c| priority(c) as i32).sum::<i32>()
            })
            .sum();

        Ok(result)
    }

    pub fn part2(_response: Response) -> Result<isize, Box<dyn std::error::Error>> {
        todo!("not doing it");
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::helper::helper::run_day;

    use super::day03::{self, priority, split_into_parts};

    const DAY: isize = 3;

    #[test]
    fn alphabet_soup() {
        let mut m = HashMap::new();
        m.insert('a', 1);
        m.insert('b', 2);
        m.insert('z', 26);
        m.insert('A', 27);
        m.insert('B', 28);
        m.insert('Y', 51);
        m.insert('Z', 52);

        for (&k, &v) in m.iter() {
            let result = priority(k);
            assert_eq!(result, v, "expected {} to return {}", k, v);
        }
    }

    #[test]
    fn split() {
        let tests = [
            ("abccba", ["abc", "cba"]),
            ("vJrwpWtwJgWrhcsFMMfFFhFp", ["vJrwpWtwJgWr", "hcsFMMfFFhFp"]),
        ];

        for (input, expected) in tests {
            let result = split_into_parts(input);
            assert_eq!(result, expected, "expected split with {}", input);
        }
    }

    #[test]
    fn part1() {
        run_day(DAY, day03::part1);
    }

    #[test]
    fn part2() {
        run_day(DAY, day03::part2);
    }
}
