#![allow(unused)] // FIXME

#[cfg(test)]
pub mod day12 {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap, HashSet},
        fmt::Debug,
    };

    use itertools::Itertools;
    use priority_queue::PriorityQueue;

    struct Grid<T>
    where
        T: std::ops::Sub<Output = T> + PartialOrd + From<isize> + Copy + Debug,
    {
        cells: Vec<Vec<T>>,
    }

    impl<T> Grid<T>
    where
        T: std::ops::Sub<Output = T> + PartialOrd + From<isize> + Copy + Debug,
    {
        fn get(&self, x: isize, y: isize) -> Option<&T> {
            let val = self.cells.get(y as usize)?.get(x as usize);

            // println!("{x},{y} = {val:?}");

            val
        }

        fn neighbors(&self, x: isize, y: isize) -> Vec<((isize, isize), &T)> {
            [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
                .iter()
                .flat_map(|&v| match self.get(v.0, v.1) {
                    Some(val) => Some((v, val)),
                    None => None,
                })
                .collect_vec()
        }

        fn connected_neighbors(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
            let my_height = match self.get(x, y) {
                Some(h) => h,
                None => panic!("no get for value {x},{y}"),
            };

            self.neighbors(x, y)
                .iter()
                .filter(|(_, &height)| {
                    let val = height - *my_height;
                    val <= 1.into()
                })
                .map(|(loc, _)| *loc)
                .collect()
        }

        fn distance(&self, current: (isize, isize), neighbor: (isize, isize)) -> T {
            *self.get(neighbor.0, neighbor.1).unwrap() - *self.get(current.0, current.1).unwrap()
        }
    }

    fn parse_board(text: &str) -> Grid<isize> {
        Grid {
            cells: text
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| {
                            if c == 'S' {
                                'a'
                            } else if c == 'E' {
                                'z'
                            } else {
                                c
                            }
                        })
                        .map(|c| (c as u8 - 'a' as u8).into())
                        .collect()
                })
                .collect(),
        }
    }

    pub fn part1(text: String) -> Result<usize, Box<dyn std::error::Error>> {
        let g = parse_board(&text[..]);

        let start = text.lines().join("").find("S").unwrap();
        let start = (
            (start % g.cells[0].len()) as isize,
            (start / g.cells[0].len()) as isize,
        );
        let end = text.lines().join("").find("E").unwrap();
        let end = (
            (end % g.cells[0].len()) as isize,
            (end / g.cells[0].len()) as isize,
        );

        let prev = djikstra(&g, start, end)?;

        let mut path: Vec<(isize, isize)> = unroll(prev, end);
        path.retain(|&e| e != start);

        Ok(path.len())
    }

    fn show_output(text: &str, path: &Vec<(isize, isize)>) -> String {
        let mut view: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();

        for &(x, y) in path.iter() {
            view[y as usize][x as usize] = '.';
        }

        let output = view.iter().map(|line| line.iter().join("")).join("\n");

        output
    }

    fn unroll(
        prev: HashMap<(isize, isize), (isize, isize)>,
        end: (isize, isize),
    ) -> Vec<(isize, isize)> {
        let mut result = vec![];

        let mut current = Some(&end);
        while let Some(u) = current {
            result.insert(0, u);
            current = prev.get(u);
        }

        result.iter().map(|&u| u.clone()).collect()
    }

    fn a_star(
        grid: Grid<isize>,
        start: (isize, isize),
        goal: (isize, isize),
    ) -> Option<Vec<(isize, isize)>> {
        let mut open_set = BinaryHeap::from([(0, start)]);

        let mut came_from: HashMap<(isize, isize), (isize, isize)> = HashMap::new();

        let mut g_score = HashMap::from([(start, 0)]);
        // let mut f_score = HashMap::from([(start, 0)]);

        while !open_set.is_empty() {
            let (_, current) = open_set.pop().unwrap();
            if current == goal {
                // this is the money
                return Some(construct_path(came_from, current));
            }

            for neighbor in grid.connected_neighbors(current.0, current.1) {
                // distance is always 1
                let tentative_score = g_score.get(&current).unwrap() + 1;
                // g_score.get(&current).unwrap() + grid.distance(current, neighbor);

                if tentative_score < *g_score.get(&neighbor).unwrap_or(&std::isize::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_score);
                    let f_score = tentative_score + h(neighbor, goal);
                    if None == open_set.iter().find(|(_, v)| *v == neighbor) {
                        open_set.push((f_score, neighbor))
                    }
                }
            }
        }

        None
    }

    fn djikstra(
        grid: &Grid<isize>,
        start: (isize, isize),
        goal: (isize, isize),
    ) -> Result<HashMap<(isize, isize), (isize, isize)>, &str> {
        let mut prev = HashMap::<(isize, isize), (isize, isize)>::new();

        let vertexes: Vec<(isize, isize)> = (0..grid.cells[0].len() as isize)
            .cartesian_product(0..grid.cells.len() as isize)
            .collect();

        let mut q = PriorityQueue::new();

        for vertex in vertexes {
            q.push(vertex, Reverse(std::isize::MAX));
        }
        q.change_priority(&start, Reverse(0));

        while !q.is_empty() {
            let (u, dist_u) = q.pop().unwrap();

            if u == goal {
                break;
            }

            if dist_u.0 == std::isize::MAX {
                // this is unknown
                // this only happens if nothing so far has found it's way to this node
                // so, the node is not connected to the starting node, and we could never
                // get here
                // this means that we need to validate that the path actually worked.
                break;
            }

            if dist_u.0 == std::isize::MAX {
                println!("{u:?}");
            }

            for v in grid.connected_neighbors(u.0, u.1) {
                if let Some((&v, &dist_v)) = q.get(&v) {
                    // distance is always 1

                    let alt = dist_u.0 + 1;
                    if alt < dist_v.0 {
                        q.change_priority(&v, Reverse(alt));
                        prev.insert(v, u);
                    }
                }
            }
        }

        if prev.contains_key(&goal) {
            Ok(prev)
        } else {
            Err("not a connected graph")
        }
    }

    fn h(neighbor: (isize, isize), goal: (isize, isize)) -> isize {
        ((goal.0 - neighbor.0).abs() + (goal.1 - neighbor.1).abs()) + 100
    }

    fn construct_path(
        came_from: HashMap<(isize, isize), (isize, isize)>,
        current: (isize, isize),
    ) -> Vec<(isize, isize)> {
        let mut current = current;
        let mut path = vec![current];
        while came_from.contains_key(&current) {
            current = *came_from.get(&current).unwrap();
            path.insert(0, current);
        }

        path
    }

    pub fn part2(text: String) -> Result<isize, Box<dyn std::error::Error>> {
        let g = parse_board(&text[..]);

        let end = text.lines().join("").find("E").unwrap();
        let end = (
            (end % g.cells[0].len()) as isize,
            (end / g.cells[0].len()) as isize,
        );

        let mut all_a = HashSet::new();

        for (y, row) in g.cells.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    all_a.insert((x as isize, y as isize));
                }
            }
        }

        let mut distances = HashSet::new();
        for start in all_a {
            let prev = djikstra(&g, start, end);

            if let Ok(prev) = prev {
                let mut path = unroll(prev, end);
                path.retain(|&v| v != start);
                distances.insert(path.len());
            }
        }

        return Ok(*distances.iter().min().unwrap() as isize);
    }

    #[cfg(test)]
    mod test {
        use crate::helper::helper::run_day;

        use super::*;

        const DAY: usize = 12;

        #[test]
        fn test_binheap() {
            let start = (0, (0, 0));
            let mut heap = BinaryHeap::from([start]);

            heap.push((11, (1, 1)));
            heap.push((22, (2, 2)));
            heap.push((1, (5, 5)));
            heap.push((2, (5, 5)));
            heap.push((11111, (5, 5)));

            while !heap.is_empty() {
                dbg!(heap.pop());
            }
        }

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
