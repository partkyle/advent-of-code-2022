#[cfg(test)]
pub mod day14 {
    use std::fmt::{Display, Formatter};

    use nom::bytes::complete::tag;
    use nom::character::complete;
    use nom::character::complete::newline;
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;

    const SAND_SPAWN: (usize, usize) = (500, 0);

    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Tile {
        Air,
        Rock,
        Sand,
    }

    impl Display for Tile {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let c = self.to_char();

            write!(f, "{}", c)
        }
    }

    impl Tile {
        fn to_char(&self) -> char {
            match *self {
                Tile::Air => {
                    '.'
                }
                Tile::Rock => {
                    '#'
                }
                Tile::Sand => {
                    'O'
                }
            }
        }
    }

    struct World {
        cells: Vec<Vec<Tile>>,
        size: usize,
        min: (usize, usize),
        max: (usize, usize),
    }

    impl World {
        fn new(size: usize) -> World {
            World {
                cells: (0..size).map(|_| (0..size).map(|_| Tile::Air).collect()).collect(),
                size,
                min: (usize::MAX, usize::MAX),
                max: (0, 0),
            }
        }

        fn get(&self, loc: (usize, usize)) -> Option<&Tile> {
            self.cells.get(loc.1)?.get(loc.0)
        }

        fn count_sands(&self) -> usize {
            self.cells.iter().map(|row| row.iter().filter(|&&t| t == Tile::Sand).count()).sum()
        }

        fn update_loc_seen(&mut self, loc: (usize, usize)) {
            self.min = (self.min.0.min(loc.0), self.min.1.min(loc.1));
            self.max = (self.max.0.max(loc.0), self.max.1.max(loc.1));
        }

        fn set(&mut self, loc: (usize, usize), tile: Tile) {
            self.update_loc_seen(loc);
            self.cells[loc.1][loc.0] = tile;
        }

        fn draw_world(&self) {
            let lines: Vec<String> = (self.min.1..=self.max.1).map(
                |y|
                    (self.min.0..=self.max.0).map(|x|
                        self.get((x, y)).unwrap().to_char()
                    ).collect()
            ).collect();

            for line in lines {
                println!("{line}");
            }
        }
    }

    fn point(input: &str) -> IResult<&str, (usize, usize)> {
        let (input, (start, end)) =
            separated_pair(complete::u32, tag(","), complete::u32)(input)?;

        Ok((input, (start as usize, end as usize)))
    }

    fn line_nodes(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
        separated_list1(tag(" -> "), point)(input)
    }

    pub fn parse_input(input: &str) -> Vec<Vec<(usize, usize)>> {
        let (_, result) = separated_list1(newline, line_nodes)(input).unwrap();
        result
    }

    fn bottom_neighbors_ordered(loc: (usize, usize)) -> [(usize, usize); 3] {
        [
            // the middle is the leftmost because we fall straight down first
            (loc.0, loc.1 + 1),
            // then we fall to the left of that
            (loc.0 - 1, loc.1 + 1),
            // then to right
            (loc.0 + 1, loc.1 + 1),
        ]
    }

    fn populate_world(text: &str, size: usize) -> World {
        let mut world = World::new(size);
        let list_of_line_nodes = parse_input(text);

        for line_node in list_of_line_nodes {
            for window in line_node.windows(2) {
                let start = window[0];
                let end = window[1];

                for x in start.0.min(end.0)..=start.0.max(end.0) {
                    world.set((x, start.1), Tile::Rock);
                }

                for y in start.1.min(end.1)..=start.1.max(end.1) {
                    world.set((start.0, y), Tile::Rock);
                }
            }
        }

        world
    }

    pub fn part1(text: String) -> Result<usize, Box<dyn std::error::Error>> {
        let mut world = populate_world(&text[..], 1000);

        world.draw_world();

        'outer: loop {
            let mut current_loc = SAND_SPAWN;
            loop {
                let next_loc = bottom_neighbors_ordered(current_loc).iter().flat_map(|&loc|
                    match world.get(loc) {
                        Some(Tile::Air) => { Some(loc) }
                        _ => None
                    }
                ).next();


                match next_loc {
                    Some(loc) => {
                        current_loc = loc;
                        if current_loc.1 > world.max.1 {
                            break 'outer;
                        }
                    }

                    None => {
                        // we are done if there is nowhere to go
                        break;
                    }
                };
            }
            world.set(current_loc, Tile::Sand);
        }

        world.draw_world();
        Ok(world.count_sands())
    }

    pub fn part2(text: String) -> Result<usize, Box<dyn std::error::Error>> {
        let size = 1000;
        let mut world = populate_world(&text[..], size);


        let old_min = world.min;
        let old_max = world.max;
        let floor_y = world.max.1 + 2;
        for x in 0..size {
            world.set((x, floor_y), Tile::Rock);
        }

        world.min = old_min;
        world.max = old_max;

        world.draw_world();


        let mut count = 0;
        'outer: loop {
            let mut current_loc = SAND_SPAWN;
            loop {
                let next_loc = bottom_neighbors_ordered(current_loc).iter().flat_map(|&loc|
                    match world.get(loc) {
                        Some(Tile::Air) => { Some(loc) }
                        _ => None
                    }
                ).next();


                match next_loc {
                    Some(loc) => {
                        current_loc = loc;
                    }

                    None => {
                        // we are done if there is nowhere to go
                        break;
                    }
                };
            }
            world.set(current_loc, Tile::Sand);
            
            if current_loc == SAND_SPAWN {
                break 'outer;
            }
        }

        println!("================================================================");
        println!("================================================================");
        println!("================================================================");
        println!("================================================================");
        world.draw_world();
        Ok(world.count_sands())
    }

    #[cfg(test)]
    mod test {
        use crate::helper::helper::{dbg_day, run_day};

        use super::*;

        const DAY: usize = 14;

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


