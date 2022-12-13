#[cfg(test)]
pub mod day07 {
    use itertools::Itertools;

    use crate::tree::tree::Tree;

    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    #[derive(Debug)]
    struct Data<S>
    where
        S: Into<String>,
    {
        name: S,
        size: usize,
    }

    impl<S> Data<S>
    where
        S: Into<String>,
    {
        pub fn new(name: S) -> Data<S> {
            Data {
                name: name,
                size: 0,
            }
        }
    }

    fn sum_children_stored(
        t: &Rc<RefCell<Tree<Data<&str>>>>,
        path: Vec<&str>,
        data: &mut HashMap<String, usize>,
    ) -> usize {
        let mut new_path = path;
        new_path.push(t.as_ref().borrow().data.name);

        let size = t.as_ref().borrow().data.size;

        let result = size
            + t.as_ref()
                .borrow()
                .children
                .iter()
                .map(|child| sum_children_stored(child, new_path.clone(), data))
                .sum::<usize>();

        data.insert(new_path.join("/"), result);

        result
    }

    fn find_file_sizes(lines: Vec<&str>) -> HashMap<String, usize> {
        let mut cursor = 0;

        let root = Tree::new(Data::new("/"));
        let mut current = Rc::clone(&root);

        while cursor < lines.len() {
            let line = lines[cursor];

            if line.starts_with("$ cd") {
                let directory_name = line.split_whitespace().last().unwrap();
                if directory_name == "/" {
                    current = Rc::clone(&root);
                } else if directory_name == ".." {
                    let new_parent = match &current.as_ref().borrow_mut().parent {
                        Some(parent) => Rc::clone(parent),
                        None => Rc::clone(&current),
                    };
                    current = new_parent;
                } else {
                    current = Tree::add_child(&current, Data::new(directory_name));
                }
                cursor += 1;
            } else if line.starts_with("$ ls") {
                let mut next_line: &str;
                cursor += 1;
                while cursor < lines.len() {
                    next_line = lines[cursor];

                    if next_line.starts_with("$") {
                        break;
                    }

                    let value = next_line.split_whitespace().next().unwrap();

                    if let Ok(size) = value.parse::<usize>() {
                        current.as_ref().borrow_mut().data.size += size;
                    }

                    cursor += 1;
                }
            } else {
                panic!("something unhandled this way comes: {}", line);
            }
        }

        let mut data: HashMap<String, usize> = HashMap::new();
        sum_children_stored(&root, vec![], &mut data);

        data
    }

    const MAX_SIZE: usize = 100000;

    pub fn part1(text: String) -> Result<usize, Box<dyn std::error::Error>> {
        let data = find_file_sizes(text.lines().collect());

        let result = data
            .iter()
            .map(|(_, &size)| size)
            .sorted()
            .filter(|&size| size <= MAX_SIZE)
            .sum();

        Ok(result)
    }

    const TOTAL_FILESYSTEM_SIZE: usize = 70000000;
    const FREE_SPACE_NEEDED: usize = 30000000;

    pub fn part2(text: String) -> Result<usize, Box<dyn std::error::Error>> {
        let data = find_file_sizes(text.lines().collect());

        let max = data
            .iter()
            .map(|(_, &i)| i)
            .max()
            .ok_or("no max value found")?;

        let current_free_space = TOTAL_FILESYSTEM_SIZE - max;
        let needed_free_space = FREE_SPACE_NEEDED - current_free_space;

        let result = data
            .iter()
            .map(|(_, &i)| i)
            .filter(|&i| i >= needed_free_space)
            .sorted()
            .next()
            .ok_or("no answer matches criteria".into());

        result
    }
}

#[cfg(test)]
mod test {
    use crate::helper::helper::run_day;

    use super::day07;

    const DAY: usize = 7;

    #[test]
    fn part1() {
        run_day(DAY, day07::part1);
    }

    #[test]
    fn part2() {
        run_day(DAY, day07::part2);
    }
}
