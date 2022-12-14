#[cfg(test)]
pub mod day11 {
    use std::collections::HashMap;

    use itertools::Itertools;

    #[derive(PartialEq, Debug, Clone, Copy)]
    enum Operation {
        Add(isize),
        Multiply(isize),
        Squared,
    }

    impl Operation {
        fn operate<T: Into<isize>>(&self, item: T) -> isize {
            match self {
                Operation::Add(i) => item.into() + i,
                Operation::Multiply(i) => item.into() * i,
                Operation::Squared => item.into().pow(2),
            }
        }
    }

    impl Default for Operation {
        fn default() -> Self {
            Operation::Add(0)
        }
    }

    #[derive(Default, Debug)]
    struct Monkey {
        id: usize,
        items: Vec<isize>,
        divisible_by: isize,
        op: Operation,
        if_true: usize,
        if_false: usize,
    }

    impl Monkey {
        fn test(&self, item: isize) -> bool {
            item % self.divisible_by == 0
        }
    }

    fn parse_monkey(monkey: &str) -> Result<Monkey, Box<dyn std::error::Error>> {
        let mut lines = monkey.lines();

        let id = lines
            .next()
            .unwrap()
            .strip_prefix("Monkey ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse()?;

        let items: Result<Vec<isize>, _> = lines
            .next()
            .ok_or("missing line item")?
            .trim()
            .trim_start_matches("Starting items: ")
            .split(",")
            .map(|s| s.trim().parse())
            .collect();

        let operation_parts: Vec<&str> = lines
            .next()
            .ok_or("missing operation")?
            .trim()
            .trim_start_matches("Operation: new =")
            .split_whitespace()
            .collect();

        let op = match &operation_parts[..] {
            ["old", "*", "old"] => Operation::Squared,
            ["old", "*", s] => Operation::Multiply(s.parse()?),
            ["old", "+", s] => Operation::Add(s.parse()?),

            _ => Err("invalid operation")?,
        };

        let divisible_by = lines
            .next()
            .ok_or("missing divisible_by")?
            .trim()
            .trim_start_matches("Test: divisible by ")
            .parse()?;

        let if_true = lines
            .next()
            .ok_or("missing if_true")?
            .trim()
            .trim_start_matches("If true: throw to monkey ")
            .parse()?;

        let if_false = lines
            .next()
            .ok_or("missing if_false")?
            .trim()
            .trim_start_matches("If false: throw to monkey ")
            .parse()?;

        Ok(Monkey {
            id: id,
            items: items?,
            op: op,
            divisible_by: divisible_by,
            if_true: if_true,
            if_false: if_false,
            ..Default::default()
        })
    }

    fn execute_round_vec(monkeys: &mut Vec<Monkey>, inspections: &mut HashMap<isize, isize>) {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items.clear();

            let items: Vec<isize> = items
                .iter()
                .map(|&item| monkeys[i].op.operate(item) / 3)
                .collect();

            for item in items {
                inspections
                    .entry(i as isize)
                    .and_modify(|e| *e = e.clone() + 1);
                let result = monkeys[i].test(item);
                let loc = if result {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[loc].items.push(item);
            }
        }
    }

    pub fn part1(text: String) -> Result<isize, Box<dyn std::error::Error>> {
        let data = text.split("\n\n");

        let monkeys: Result<Vec<Monkey>, _> = data.map(parse_monkey).collect();
        let mut monkeys = monkeys?;

        let mut inspections: HashMap<isize, isize> =
            HashMap::from_iter((0..monkeys.len()).map(|i| (i as isize, 0)));

        let rounds = 20;

        for _ in 0..rounds {
            execute_round_vec(&mut monkeys, &mut inspections);
        }

        let result: isize = inspections
            .iter()
            .map(|(_, v)| v)
            .sorted()
            .rev()
            .take(2)
            .product();

        Ok(result)
    }

    pub fn part2(_text: String) -> Result<isize, Box<dyn std::error::Error>> {
        todo!("do it");
    }

    #[cfg(test)]
    mod test {
        use crate::helper::helper::run_day;

        use super::*;

        const DAY: usize = 11;

        const MONKEY: &str = r#"Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3"#;

        #[test]
        fn test_parse_monkey() {
            let monkey = parse_monkey(MONKEY).unwrap();

            assert_eq!(monkey.id, 0);
            assert_eq!(monkey.items, vec![79, 98]);
            assert_eq!(monkey.op, Operation::Multiply(19));
            assert_eq!(monkey.divisible_by, 23);
            assert_eq!(monkey.if_true, 2);
            assert_eq!(monkey.if_false, 3);
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
