use std::iter::once;
#[derive(Debug, Clone)]
pub enum Value {
    Int(usize),
    List(Vec<Box<Value>>),
}
use Value::*;

impl Value {
    pub fn is_less(&self, right: &Value) -> Option<bool> {
        match (self, right) {
            (Int(l), Int(r)) if l != r => Some(l < r),
            (Int(_), Int(_)) => None,
            (Int(l), List(_)) => List(vec![Box::new(Int(*l))]).is_less(right),
            (List(_), Int(r)) => self.is_less(&List(vec![Box::new(Int(*r))])),
            (List(l), List(r)) => {
                for i in 0..l.len() {
                    if r.get(i).is_none() {
                        return Some(false);
                    }
                    if let Some(res) = l.get(i).unwrap().is_less(r.get(i).unwrap()) {
                        return Some(res);
                    }
                }
                if l.len() == r.len() {
                    None
                } else {
                    Some(true)
                }
            }
        }
    }
}

pub fn parse(input: &str) -> Value {
    if let Ok(n) = input.parse::<usize>() {
        return Int(n);
    }
    let (mut p, mut o) = (0, 0);
    let list = input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match c {
            '[' => {
                o += 1;
                None
            }
            ']' => {
                if o == 1 {
                    let elem = parse(&input[(p + 1)..i]);
                    return Some(Box::new(elem));
                }
                o -= 1;
                None
            }
            ',' if o == 1 => {
                let elem = parse(&input[(p + 1)..i]);
                p = i;
                Some(Box::new(elem))
            }
            _ => None,
        })
        .collect();
    List(list)
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<(Value, Value)> {
    input
        .split("\n\n")
        .map(|p| {
            let mut lines = p.lines();
            let l1 = lines.next().unwrap();
            let l2 = lines.next().unwrap();
            (parse(l1), parse(l2))
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &[(Value, Value)]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if p.0.is_less(&p.1).unwrap() {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[(Value, Value)]) -> usize {
    let mut pkts: Vec<&Value> = input
        .iter()
        .flat_map(|p| once(&p.0).chain(once(&p.1)))
        .collect();

    let two = parse("[[2]]");
    let six = parse("[[6]]");
    pkts.push(&two);
    pkts.push(&six);

    pkts.sort_by(|a, b| {
        if a.is_less(b).unwrap() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    pkts.iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if two.is_less(v).is_none() || six.is_less(v).is_none() {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 13)
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 140)
    }
}
