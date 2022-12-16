#[cfg(test)]
pub mod day13 {
    use core::panic;
    use std::cmp::Ordering;

    use serde_json::Value;

    fn compare_str(a: &str, b: &str) -> Result<Ordering, Box<dyn std::error::Error>> {
        let a: serde_json::Result<Value> = serde_json::from_str(a);
        let a = a?;
        let b: serde_json::Result<Value> = serde_json::from_str(b);
        let b = b?;

        if let (Value::Array(a), Value::Array(b)) = (a, b) {
            return Ok(compare_vec(&a, &b));
        }

        Err("invalid input".into())
    }

    fn compare_vec(a: &Vec<Value>, b: &Vec<Value>) -> Ordering {
        for (left, right) in a.iter().zip(b.iter()) {
            let cmp = match (left, right) {
                (Value::Null, Value::Null) => Ordering::Equal,

                (Value::Null, _) => Ordering::Less,

                (Value::Number(l), Value::Number(r)) => {
                    l.as_i64().unwrap().cmp(&r.as_i64().unwrap())
                }

                (Value::Array(l), Value::Array(r)) => compare_vec(l, r),

                (Value::Array(l), Value::Number(r)) => {
                    compare_vec(l, &vec![Value::Number(r.clone())])
                }

                (Value::Number(l), Value::Array(r)) => {
                    compare_vec(&vec![Value::Number(l.clone())], r)
                }

                v => panic!("not handled. they lied about the ruleset {v:?}"),
            };

            if cmp != Ordering::Equal {
                return cmp;
            }
        }

        // we compare left to right, so if there are still values on the right (ie. (_, Null) in the match above)
        // then we shoudl compare the sizes of the Vec to ensure that there are less on the left hand side.
        // otherwise, everything up to here was Equal
        a.len().cmp(&b.len())
    }

    pub fn part1(text: String) -> Result<usize, Box<dyn std::error::Error>> {
        let mut results = vec![];
        for chunks in text.split("\n\n") {
            let pieces: Vec<&str> = chunks.lines().collect();

            let result = compare_str(pieces[0], pieces[1])?;
            results.push(result);
        }

        let correctly_ordered = results
            .iter()
            .enumerate()
            .map(|(i, v)| (i + 1, v))
            .filter(|(_, &v)| v == Ordering::Less);

        let sum: usize = correctly_ordered.map(|(i, _)| i).sum();
        Ok(sum)
    }

    fn parse_serde_json_array(s: &str) -> Vec<Value> {
        match serde_json::from_str(s).unwrap() {
            Value::Array(a) => a,
            _ => panic!("this is not possible"),
        }
    }

    pub fn part2(text: String) -> Result<usize, Box<dyn std::error::Error>> {
        let mut list: Vec<Vec<Value>> = text
            .lines()
            .filter(|&s| s != "")
            .map(|s| parse_serde_json_array(s))
            .collect();

        let marker1_str = "[[2]]";
        let marker1 = parse_serde_json_array(marker1_str);
        let marker2_str = "[[6]]";
        let marker2 = parse_serde_json_array(marker2_str);

        list.push(marker1);
        list.push(marker2);

        list.sort_by(|a, b| compare_vec(a, b));

        let json_list: Vec<String> = list
            .iter()
            .map(|l| serde_json::to_string(l).unwrap())
            .collect();

        // the "index" is a 1-based index
        let index_marker1 = json_list.iter().position(|s| s == marker1_str).unwrap() + 1;
        let index_marker2 = json_list.iter().position(|s| s == marker2_str).unwrap() + 1;

        Ok(index_marker1 * index_marker2)
    }

    #[cfg(test)]
    mod test {
        use crate::helper::helper::run_day;

        use super::*;

        const DAY: usize = 13;

        #[test]
        fn test_thing() {
            let val: serde_json::Result<Value> = serde_json::from_str("[]");
            let val = val.unwrap();

            dbg!(&val);

            dbg!(&val[0]);
            dbg!(&val[1]);
        }

        #[test]
        fn test_compares() {
            let tests = [
                (("[1,1,3,1,1]", "[1,1,5,1,1]"), Ordering::Less),
                (("[1,3,4]", "[4,5,9]"), Ordering::Less),
                (("[10]", "[4,5,9]"), Ordering::Greater),
                (("[]", "[1]"), Ordering::Less),
                (("[9]", "[[8,7,6]]"), Ordering::Greater),
                (("[[4,4],4,4]", "[[4,4],4,4,4]"), Ordering::Less),
                (("[7,7,7,7]", "[7,7,7]"), Ordering::Greater),
                (("[[1]]", "[1]"), Ordering::Equal),
                (("[[2]]", "[1]"), Ordering::Greater),
                (("[1,[1,2,3]]", "[1,[1,2,3,5]]"), Ordering::Less),
                (("[1,[1,2,3],4]", "[1,[1,2,3],[5]]"), Ordering::Less),
                (("[[[]]]", "[[]]"), Ordering::Greater),
                (
                    ("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"),
                    Ordering::Greater,
                ),
            ];

            for ((a, b), ord) in tests {
                let result = compare_str(a, b);
                let result = result.unwrap();
                assert_eq!(result, ord, "{a} {:?} {b}", result);
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
