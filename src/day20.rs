use std::collections::VecDeque;

pub fn mix(input: &[isize], n: usize) -> isize {
    let mut list: VecDeque<(usize, isize)> = input.iter().copied().enumerate().collect();

    for _ in 0..n {
        for i in 0..input.len() {
            while list[0].0 != i {
                list.rotate_left(1);
            }

            let val = list.pop_front().unwrap();
            let r = val.1 % (input.len() as isize - 1);
            if r > 0 {
                list.rotate_left(r as usize);
            } else {
                list.rotate_right(r.abs() as usize);
            }
            list.push_front(val);
        }

        while list[0].1 != 0 {
            list.rotate_left(1)
        }
    }

    list[1000 % list.len()].1 + list[2000 % list.len()].1 + list[3000 % list.len()].1
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse::<isize>().unwrap()).collect()
}

#[aoc(day20, part1)]
pub fn part1(input: &[isize]) -> isize {
    mix(input, 1)
}

#[aoc(day20, part2)]
pub fn part2(input: &[isize]) -> isize {
    let input: Vec<isize> = input.iter().map(|i| i * 811589153).collect();
    mix(&input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1
2
-3
3
-2
0
4";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 3)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 1623178306)
    }
}
