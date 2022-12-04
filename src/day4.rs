pub type Pair = ((usize, usize), (usize, usize));

fn is_contained(p: &Pair) -> bool {
    if p.0 .0 >= p.1 .0 && p.0 .1 <= p.1 .1 {
        return true;
    } else if p.1 .0 >= p.0 .0 && p.1 .1 <= p.0 .1 {
        return true;
    }
    false
}

fn overlap(p: &Pair) -> bool {
    if p.0 .1 >= p.1 .0 && p.0 .0 <= p.1 .1 {
        return true;
    } else if p.0 .1 <= p.1 .0 && p.0 .0 >= p.1 .1 {
        return true;
    }
    false
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(",").unwrap();
            let (x, y) = a.split_once("-").unwrap();
            let (w, z) = b.split_once("-").unwrap();
            (
                (x.parse().unwrap(), y.parse().unwrap()),
                (w.parse().unwrap(), z.parse().unwrap()),
            )
        })
        .collect()
}
#[aoc(day4, part1)]
pub fn part1(input: &[Pair]) -> usize {
    input
        .into_iter()
        .fold(0, |acc, p| if is_contained(p) { acc + 1 } else { acc })
}
#[aoc(day4, part2)]
pub fn part2(input: &[Pair]) -> usize {
    input
        .into_iter()
        .fold(0, |acc, p| if overlap(p) { acc + 1 } else { acc })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            )),
            2
        )
    }
    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            )),
            4
        )
    }
}
