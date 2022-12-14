#[cfg(test)]
pub mod day09 {
    use std::{collections::HashSet, str::FromStr};

    #[derive(Debug)]
    enum Direction {
        Up(usize),
        Down(usize),
        Left(usize),
        Right(usize),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Coord {
        x: isize,
        y: isize,
    }

    fn execute_move(
        dir: &Direction,
        head: &mut Coord,
        tail: &mut Coord,
        tail_positions: &mut HashSet<Coord>,
    ) {
        match dir {
            Direction::Up(count) => {
                for _ in 0..*count {
                    head.y += 1;
                    move_tail(head, tail);
                    tail_positions.insert(tail.clone());
                }
            }
            Direction::Down(count) => {
                for _ in 0..*count {
                    head.y -= 1;
                    move_tail(head, tail);
                    tail_positions.insert(tail.clone());
                }
            }
            Direction::Left(count) => {
                for _ in 0..*count {
                    head.x -= 1;
                    move_tail(head, tail);
                    tail_positions.insert(tail.clone());
                }
            }
            Direction::Right(count) => {
                for _ in 0..*count {
                    head.x += 1;
                    move_tail(head, tail);
                    tail_positions.insert(tail.clone());
                }
            }
        }
    }

    fn execute_move_chain(
        dir: &Direction,
        snake: &mut Vec<Coord>,
        tail_positions: &mut HashSet<Coord>,
    ) {
        match dir {
            Direction::Up(count) => {
                for _ in 0..*count {
                    snake[0].y += 1;
                    for i in 0..snake.len() - 1 {
                        move_tail(&snake[i].clone(), &mut snake[i + 1])
                    }
                    tail_positions.insert(snake[snake.len() - 1].clone());
                }
            }
            Direction::Down(count) => {
                for _ in 0..*count {
                    snake[0].y -= 1;
                    for i in 0..snake.len() - 1 {
                        move_tail(&snake[i].clone(), &mut snake[i + 1])
                    }
                    tail_positions.insert(snake[snake.len() - 1].clone());
                }
            }
            Direction::Left(count) => {
                for _ in 0..*count {
                    snake[0].x -= 1;
                    for i in 0..snake.len() - 1 {
                        move_tail(&snake[i].clone(), &mut snake[i + 1])
                    }
                    tail_positions.insert(snake[snake.len() - 1].clone());
                }
            }
            Direction::Right(count) => {
                for _ in 0..*count {
                    snake[0].x += 1;
                    for i in 0..snake.len() - 1 {
                        move_tail(&snake[i].clone(), &mut snake[i + 1])
                    }
                    tail_positions.insert(snake[snake.len() - 1].clone());
                }
            }
        }
    }

    fn move_tail(head: &Coord, tail: &mut Coord) {
        if head.x != tail.x && head.y != tail.y {
            let diff_x = head.x - tail.x;
            let diff_y = head.y - tail.y;

            if diff_x.abs() >= 2 || diff_y.abs() >= 2 {
                tail.x += if diff_x < 0 { -1 } else { 1 };
                tail.y += if diff_y < 0 { -1 } else { 1 };
            }
        } else if head.x != tail.x {
            let diff = head.x - tail.x;
            if diff.abs() >= 2 {
                tail.x += if diff < 0 { -1 } else { 1 }
            }
        } else if head.y != tail.y {
            let diff = head.y - tail.y;
            if diff.abs() >= 2 {
                tail.y += if diff < 0 { -1 } else { 1 }
            }
        }
    }

    impl FromStr for Direction {
        type Err = Box<dyn std::error::Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split_whitespace();

            let dir = parts.next().ok_or("missing direction")?;
            let num = parts.next().ok_or("missing amount")?;

            let result = match dir {
                "U" => Direction::Up(num.parse()?),
                "D" => Direction::Down(num.parse()?),
                "L" => Direction::Left(num.parse()?),
                "R" => Direction::Right(num.parse()?),

                _ => Err("invalid direction")?,
            };

            Ok(result)
        }
    }

    pub fn part1(text: String) -> Result<usize, Box<dyn std::error::Error>> {
        let dirs: Result<Vec<Direction>, _> = text.lines().map(|c| c.parse()).collect();

        let start = Coord { x: 0, y: 0 };
        let mut head = start.clone();
        let mut tail = start.clone();

        let mut tail_positions = HashSet::new();

        for dir in dirs? {
            execute_move(&dir, &mut head, &mut tail, &mut tail_positions);
        }

        Ok(tail_positions.len())
    }

    pub fn part2(text: String) -> Result<usize, Box<dyn std::error::Error>> {
        let dirs: Result<Vec<Direction>, _> = text.lines().map(|c| c.parse()).collect();

        let start = Coord { x: 0, y: 0 };
        let mut snake: Vec<Coord> = (0..10).map(|_| start.clone()).collect();

        let mut tail_positions = HashSet::new();

        for dir in dirs? {
            execute_move_chain(&dir, &mut snake, &mut tail_positions);
        }

        Ok(tail_positions.len())
    }
}

#[cfg(test)]
mod test {
    use crate::helper::helper::run_day;

    use super::day09;

    const DAY: usize = 9;

    #[test]
    fn part1() {
        run_day(DAY, day09::part1);
    }

    #[test]
    fn part2() {
        run_day(DAY, day09::part2);
    }
}
