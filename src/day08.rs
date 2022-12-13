#[cfg(test)]
pub mod day08 {
    use std::fmt::Debug;

    #[derive(Debug)]
    pub struct Grid<T: Debug + PartialOrd> {
        cells: Vec<Vec<T>>,
    }

    impl<T: Debug + PartialOrd> Grid<T> {
        pub fn get<'a>(&'a self, x: usize, y: usize) -> Option<&'a T> {
            self.cells.get(y)?.get(x)
        }

        pub fn height(&self) -> usize {
            self.cells.len()
        }

        pub fn width(&self) -> usize {
            self.cells[0].len()
        }

        pub fn north<'a>(&'a self, x: usize, y: usize) -> impl Iterator<Item = &'a T> {
            (0..y).rev().flat_map(move |i| self.get(x, i))
        }

        pub fn south<'a>(&'a self, x: usize, y: usize) -> impl Iterator<Item = &'a T> {
            (y + 1..self.height()).flat_map(move |i| self.get(x, i))
        }

        pub fn east<'a>(&'a self, x: usize, y: usize) -> impl Iterator<Item = &'a T> {
            (x + 1..self.width()).flat_map(move |i| self.get(i, y))
        }

        pub fn west<'a>(&'a self, x: usize, y: usize) -> impl Iterator<Item = &'a T> {
            (0..x).rev().flat_map(move |i| self.get(i, y))
        }

        pub fn hidden(&self, x: usize, y: usize) -> bool {
            let value = self.get(x, y).unwrap();

            self.north(x, y).any(|i| i >= value)
                && self.south(x, y).any(|i| i >= value)
                && self.east(x, y).any(|i| i >= value)
                && self.west(x, y).any(|i| i >= value)
        }

        // take until we see one that is our height or higher
        // we can't use take_while, because we still want the last one
        // if it's larger than the first
        fn take_until<'a, U: 'a + PartialOrd>(
            iter: &mut impl Iterator<Item = &'a U>,
            my_height: &U,
        ) -> Vec<&'a U> {
            let mut result = vec![];

            for i in iter {
                result.push(i);

                if my_height <= i {
                    break;
                }
            }

            result
        }

        pub fn scenic_score(&self, x: usize, y: usize) -> usize {
            let value = self.get(x, y).unwrap();

            let north_line: Vec<&T> = Grid::<T>::take_until(&mut self.north(x, y), value);
            let south_line: Vec<&T> = Grid::<T>::take_until(&mut self.south(x, y), value);
            let east_line: Vec<&T> = Grid::<T>::take_until(&mut self.east(x, y), value);
            let west_line: Vec<&T> = Grid::<T>::take_until(&mut self.west(x, y), value);

            north_line.len() * south_line.len() * east_line.len() * west_line.len()
        }
    }

    type Tree = i32;

    pub fn new_from_string<S: Into<String>>(
        text: S,
    ) -> Result<Grid<Tree>, Box<dyn std::error::Error>> {
        let cells: Result<Vec<Vec<Tree>>, _> = text
            .into()
            .lines()
            .map(|line| line.chars().map(|c| c.to_string().parse()).collect())
            .collect();

        let grid = Grid { cells: cells? };

        Ok(grid)
    }

    pub fn part1(text: String) -> Result<isize, Box<dyn std::error::Error>> {
        let grid = new_from_string(text)?;

        let mut count = 0;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if !grid.hidden(x, y) {
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    pub fn part2(text: String) -> Result<usize, Box<dyn std::error::Error>> {
        let grid = new_from_string(text)?;

        let mut max = 0;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let score = grid.scenic_score(x, y);
                if score > max {
                    max = score;
                }
            }
        }

        Ok(max)
    }
}

#[cfg(test)]
mod test {
    use crate::helper::helper::run_day;

    use super::day08::{self, new_from_string};

    const DAY: usize = 8;

    fn example() -> impl Into<String> {
        ["30373", "25512", "65332", "33549", "35390"].join("\n")
    }

    #[test]
    fn visible() {
        let grid = new_from_string(example()).unwrap();

        let tests = [
            // edges
            ((0, 0), true),
            ((0, 1), true),
            ((4, 4), true),
            ((1, 1), true),
            ((3, 3), false),
        ];

        for ((x, y), expected) in tests {
            let actual = grid.hidden(x, y);
            assert_eq!(
                !actual, expected,
                "expected ({},{}) to be {}",
                x, y, expected
            );
        }
    }

    #[test]
    fn scenic() {
        let grid = new_from_string(example()).unwrap();

        let tests = [((0, 0), 0), ((2, 1), 4), ((2, 3), 8)];

        for ((x, y), expected) in tests {
            let actual = grid.scenic_score(x, y);
            println!("{:?}", grid.get(x, y));
            assert_eq!(actual, expected, "wrong score for ({},{})", x, y);
        }
    }

    #[test]
    fn part1() {
        run_day(DAY, day08::part1);
    }

    #[test]
    fn part2() {
        run_day(DAY, day08::part2);
    }
}
