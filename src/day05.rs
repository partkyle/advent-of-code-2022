#[cfg(test)]
pub mod day05 {
    use reqwest::blocking::Response;

    fn create_boxes(boxes_str: &str) -> Vec<Vec<char>> {
        let boxes: Vec<Vec<char>> = boxes_str.lines().map(|s| s.chars().collect()).collect();

        let mut stacks = vec![];
        let distance_between = "] [".len() + 1;
        let line_length = boxes[0].len();
        let mut index = 1;

        while index < line_length {
            let mut v = vec![];
            for line in &boxes[0..boxes.len() - 1] {
                let c = line[index];
                if c != ' ' {
                    // insert in front because we are loading them in backwards from the text
                    // and we want to use pop later
                    v.insert(0, c);
                }
            }
            stacks.push(v);
            index += distance_between;
        }

        stacks
    }

    #[derive(Debug)]
    struct Instruction {
        count: usize,
        src: usize,
        dest: usize,
    }

    fn parse_instruction(line: &str) -> Result<Instruction, Box<dyn std::error::Error>> {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        Ok(Instruction {
            count: tokens[1].parse()?,
            src: tokens[3].parse()?,
            dest: tokens[5].parse()?,
        })
    }

    fn get_boxes_and_instructions(
        text: String,
    ) -> Result<(Vec<Vec<char>>, Vec<Instruction>), Box<dyn std::error::Error>> {
        let mut texts = text.split("\n\n");
        let boxes_str = texts.next().unwrap();
        let instructions_str = texts.next().unwrap();

        let stacks = create_boxes(boxes_str);
        let instructions: Result<Vec<Instruction>, _> =
            instructions_str.lines().map(parse_instruction).collect();

        Ok((stacks, instructions.map_err(|e| e.to_string())?))
    }

    pub fn part1(response: Response) -> Result<String, Box<dyn std::error::Error>> {
        let text = response.text()?;
        let (mut stacks, instructions) = get_boxes_and_instructions(text)?;

        for inst in instructions {
            for _ in 0..inst.count {
                let item = stacks[inst.src - 1].pop().unwrap();
                stacks[inst.dest - 1].push(item);
            }
        }

        let result: String = stacks.iter().map(|s| s.last().unwrap()).collect();

        Ok(result)
    }

    pub fn part2(response: Response) -> Result<String, Box<dyn std::error::Error>> {
        let text = response.text()?;
        let (mut stacks, instructions) = get_boxes_and_instructions(text)?;

        for inst in instructions {
            let mut boxes = vec![];
            for _ in 0..inst.count {
                let item = stacks[inst.src - 1].pop().unwrap();
                boxes.insert(0, item);
            }
            stacks[inst.dest - 1].append(&mut boxes);
        }

        let result: String = stacks.iter().map(|s| s.last().unwrap()).collect();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::helper::helper::run_day;

    use super::day05;

    const DAY: isize = 5;

    #[test]
    fn dataset() {
        run_day(DAY, |r| Ok(r.text().unwrap()));
    }

    #[test]
    fn part1() {
        run_day(DAY, day05::part1);
    }

    #[test]
    fn part2() {
        run_day(DAY, day05::part2);
    }
}
