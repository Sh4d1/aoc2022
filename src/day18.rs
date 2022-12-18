use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point(isize, isize, isize);

impl Point {
    pub fn neighbours(&self) -> Vec<Point> {
        vec![
            Point(self.0 - 1, self.1, self.2),
            Point(self.0 + 1, self.1, self.2),
            Point(self.0, self.1 - 1, self.2),
            Point(self.0, self.1 + 1, self.2),
            Point(self.0, self.1, self.2 - 1),
            Point(self.0, self.1, self.2 + 1),
        ]
    }

    pub fn is_in_max(&self, max: (isize, isize, isize)) -> bool {
        self.0 >= -1
            && self.0 <= max.0 + 1
            && self.1 >= -1
            && self.1 <= max.1 + 1
            && self.2 >= -1
            && self.2 <= max.2 + 1
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> HashSet<Point> {
    input
        .lines()
        .map(|l| {
            let c: Vec<isize> = l.split(",").map(|c| c.parse::<isize>().unwrap()).collect();
            Point(c[0], c[1], c[2])
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &HashSet<Point>) -> usize {
    input
        .iter()
        .map(|c| {
            c.neighbours()
                .iter()
                .filter(|p| !input.contains(&p))
                .count()
        })
        .sum()
}
#[aoc(day18, part2)]
pub fn part2(input: &HashSet<Point>) -> usize {
    let max = input.iter().fold((0, 0, 0), |mut acc, c| {
        acc.0 = std::cmp::max(acc.0, c.0);
        acc.1 = std::cmp::max(acc.1, c.1);
        acc.2 = std::cmp::max(acc.2, c.2);
        acc
    });

    let start = Point(0, 0, 0);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(start);

    while let Some(p) = queue.pop_front() {
        if !visited.insert(p) {
            continue;
        }

        p.neighbours().iter().for_each(|n| {
            if n.is_in_max(max) && !input.contains(&n) {
                queue.push_back(*n);
            }
        })
    }

    input
        .iter()
        .map(|c| {
            c.neighbours()
                .iter()
                .filter(|p| visited.contains(p))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 64)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 58)
    }
}
