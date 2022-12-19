#[cfg(test)]
pub mod day15 {
    use std::collections::HashMap;
    use std::ops::RangeInclusive;
    use std::result;

    use itertools::Itertools;
    use nom::character::complete::{self, newline};
    use nom::multi::separated_list1;
    use nom::{bytes::complete::tag, IResult};

    fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }

    fn return_points_within_manhattan_distance(
        sensor: (i32, i32),
        beacon: (i32, i32),
    ) -> Vec<(i32, i32)> {
        let dist = distance(sensor, beacon);
        (-dist..=dist)
            .cartesian_product(-dist..=dist)
            .map(|loc| (loc.0 + sensor.0, loc.1 + sensor.1))
            .filter(|&loc| loc != sensor)
            .filter(|&loc| loc != beacon)
            .filter(|&loc| distance(loc, sensor) <= dist)
            .collect()
    }

    fn return_points_within_manhattan_distance2(
        sensor: (i32, i32),
        beacon: (i32, i32),
    ) -> Vec<(i32, i32)> {
        let dist = distance(sensor, beacon);
        let mut result = vec![];

        let mut row_count = 0;
        let mut row_count_step = 1;

        for y in (sensor.1 - dist)..=(sensor.1 + dist) {
            for x in (sensor.0 - row_count)..=(sensor.0 + row_count) {
                result.push((x, y));
            }
            if row_count == dist {
                row_count_step = -1;
            }
            row_count += row_count_step;
        }

        result
    }

    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    enum Tile {
        Sensor,
        Beacon,
        Scan,
        Empty,
    }

    impl Tile {
        fn char(self) -> char {
            match self {
                Tile::Sensor => 'S',
                Tile::Beacon => 'B',
                Tile::Scan => '#',
                Tile::Empty => '.',
            }
        }
    }

    struct World {
        map: HashMap<(i32, i32), Tile>,
        min: (i32, i32),
        max: (i32, i32),
    }

    impl World {
        fn new() -> World {
            World {
                map: HashMap::new(),
                min: (std::i32::MAX, std::i32::MAX),
                max: (std::i32::MIN, std::i32::MIN),
            }
        }

        fn print(&self, min: (i32, i32), max: (i32, i32)) {
            println!("{:?} => {:?}", min, max);
            for y in min.1..=max.1 {
                for x in min.0..=max.0 {
                    print!("{}", self.map.get(&(x, y)).unwrap_or(&Tile::Empty).char())
                }
                println!();
            }
        }

        fn draw_world(&self) {
            self.print(self.min, self.max);
        }

        fn set(&mut self, point: (i32, i32), tile: Tile) {
            self.min = (self.min.0.min(point.0), self.min.1.min(point.1));
            self.max = (self.max.0.max(point.0), self.max.1.max(point.1));
            if !self.map.contains_key(&point) {
                self.map.insert(point, tile);
            }
        }
    }

    fn parse_x_y(input: &str) -> IResult<&str, (i32, i32)> {
        let (input, _) = tag("x=")(input)?;
        let (input, x) = complete::i32(input)?;
        let (input, _) = tag(", y=")(input)?;
        let (input, y) = complete::i32(input)?;

        Ok((input, (x, y)))
    }

    fn parse_line(input: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
        let (input, _) = tag("Sensor at ")(input)?;
        let (input, sensor) = parse_x_y(input)?;
        let (input, _) = tag(": closest beacon is at ")(input)?;
        let (input, beacon) = parse_x_y(input)?;

        Ok((input, (sensor, beacon)))
    }

    fn parse_input(
        input: &str,
    ) -> Result<Vec<((i32, i32), (i32, i32))>, Box<dyn std::error::Error>> {
        // TODO: why do I have to unwrap here
        let (_, result) = separated_list1(newline, parse_line)(input).unwrap();

        Ok(result)
    }

    #[derive(Debug)]
    struct Tworld {
        sensors: Vec<Circle>,
        beacons: Vec<(i32, i32)>,
        xrange: (i32, i32),
    }

    fn merge_ranges(ranges: Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
        let mut ranges = ranges.clone();
        ranges.sort_by(|a, b| a.start().cmp(b.start()));

        let initial = ranges[0].clone();
        let mut result = vec![initial];

        for range in ranges[1..].iter() {
            let j = result.len() - 1;

            if result[j].contains(range.start()) {
                result[j] = *result[j].start()..=*result[j].end().max(range.end());
            } else {
                result.push(range.clone());
            }
        }

        result
    }

    impl Tworld {
        fn new(input: Vec<((i32, i32), (i32, i32))>) -> Tworld {
            let mut sensors = vec![];
            let mut beacons = vec![];
            let mut xrange = (i32::MAX, i32::MIN);

            for (sensor, beacon) in input {
                let circle = Circle {
                    pos: sensor,
                    radius: distance(sensor, beacon),
                };

                xrange.0 = xrange.0.min(circle.pos.0 - circle.radius);
                xrange.1 = xrange.1.max(circle.pos.0 + circle.radius);

                sensors.push(circle);
                beacons.push(beacon);
            }

            Tworld {
                sensors,
                beacons,
                xrange,
            }
        }

        fn row_range(&self, row: i32) -> Vec<RangeInclusive<i32>> {
            let mut ranges = vec![];

            for c in self.sensors.iter() {
                let manhattan_remainder = (c.pos.1 - row).abs();
                let manhattan_diff = c.radius - manhattan_remainder;
                if manhattan_diff >= 0 {
                    ranges.push(c.pos.0 - manhattan_diff..=c.pos.0 + manhattan_diff)
                }
            }

            merge_ranges(ranges)
        }

        fn collides(&self, pos: (i32, i32)) -> bool {
            for c in self.sensors.iter() {
                if distance(c.pos, pos) <= c.radius {
                    return true;
                }
            }
            false
        }

        fn known_empty(&self, pos: (i32, i32)) -> bool {
            !self.beacons.contains(&pos)
                && !self.sensors.iter().any(|c| c.pos == pos)
                && self.collides(pos)
        }
    }

    #[derive(Debug)]
    struct Circle {
        pos: (i32, i32),
        radius: i32,
    }

    pub fn part1_old(text: String) -> Result<usize, Box<dyn std::error::Error>> {
        let mut world = World::new();
        let y = 2_000_000;

        let input = parse_input(&text[..])?;

        println!("parsed input");

        for &(sensor, beacon) in &input {
            let dist = distance(sensor, beacon);
            // we only care if this sensor is within dist of y=2_000_000
            dbg!(sensor.1 - dist, sensor.1 + dist);
            if sensor.1 - dist <= y && y <= sensor.1 + dist {
                println!("placing sensor {sensor:?} with beacon {beacon:?} dist={dist}");
                world.set(sensor, Tile::Sensor);
                world.set(beacon, Tile::Beacon);
            }
        }

        for &(sensor, beacon) in &input {
            println!("mapping {sensor:?} to {beacon:?}");
            let points = return_points_within_manhattan_distance2(sensor, beacon);
            for point in points {
                world.set(point, Tile::Scan);
            }
        }

        world.draw_world();

        println!("done mapping sensores");

        let count = world
            .map
            .iter()
            .filter(|&(key, value)| key.1 == y && value == &Tile::Scan)
            .count();

        Ok(count)
    }

    pub fn part1(text: String) -> Result<usize, Box<dyn std::error::Error>> {
        let input = parse_input(&text[..])?;
        let world = Tworld::new(input);

        let y = 2_000_000;

        let results = (world.xrange.0..=world.xrange.1)
            .map(|i| (i, y))
            .filter(|&pos| world.known_empty(pos))
            .count();

        Ok(results)
    }

    struct Rworld {
        rows: HashMap<i32, Vec<RangeInclusive<i32>>>,
    }

    impl Rworld {
        fn new() -> Rworld {
            Rworld {
                rows: HashMap::new(),
            }
        }

        fn add_sensor_and_beacon(&mut self, sensor: (i32, i32), beacon: (i32, i32)) {
            let dist = distance(sensor, beacon);
            for y in (sensor.1 - dist)..=(sensor.1 + dist) {
                if !self.rows.contains_key(&y) {
                    self.rows.insert(y, Vec::new());
                }

                let remainder = dist - (sensor.1 - y).abs();

                let range = (sensor.0 - remainder)..=(sensor.0 + remainder);

                self.rows.entry(y).and_modify(|e| e.push(range));
            }
        }

        fn collapse_ranges(&mut self) {
            let mut new_rows = HashMap::new();
            for (key, value) in self.rows.iter() {
                let new_ranges = merge_ranges(value.clone());
                new_rows.insert(*key, new_ranges);
            }

            self.rows = new_rows;
        }
    }

    pub fn part2(text: String) -> Result<i64, Box<dyn std::error::Error>> {
        let input = parse_input(&text[..])?;

        let mut world = Rworld::new();

        for (sensor, beacon) in input {
            world.add_sensor_and_beacon(sensor, beacon);
        }

        world.collapse_ranges();

        let comparison_range = (0..=4_000_000);
        let mut row_found = None;

        for row in 0..4_000_000 {
            let vec = vec![];
            let row_ranges = world.rows.get(&row).unwrap_or(&vec);

            if row_ranges.iter().all(|r| {
                !(r.start() <= comparison_range.start() && comparison_range.end() <= r.end())
            }) {
                row_found = Some(row);
                break;
            }
        }

        let row_found = row_found.unwrap();
        let row_range = world.rows.get(&row_found).unwrap();

        dbg!(row_found, row_range);

        let xresult = comparison_range
            .filter(|x| row_range.iter().all(|r| !r.contains(x)))
            .next()
            .unwrap() as i64;

        Ok(xresult * 4_000_000 + row_found as i64)
    }

    #[cfg(test)]
    mod test {
        use crate::helper::helper::{dbg_day, run_day};

        use super::*;

        const DAY: usize = 15;

        #[test]
        fn test_ranges() {
            let ranges = vec![0..=10, 6..=9, 11..=14, 15..=20, 4..=12, 13..=15, -2..=2];

            dbg!(&ranges);

            let merged = merge_ranges(ranges);

            dbg!(&merged);
        }

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
