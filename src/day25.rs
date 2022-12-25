pub fn to_snafu(n: usize) -> String {
    let mut tmp = n;
    let mut res = String::new();
    while tmp != 0 {
        let (mut d, r) = (tmp / 5, tmp % 5);
        match r {
            3 => {
                res.push('=');
                d += 1
            }
            4 => {
                res.push('-');
                d += 1
            }
            n => res.push_str(&n.to_string()),
        }
        tmp = d;
    }
    res.chars().rev().collect()
}

pub fn from_snafu(s: &String) -> usize {
    let mut res = 0;
    for (i, c) in s.as_bytes().iter().rev().enumerate() {
        let n = match *c as char {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => unreachable!(),
        };
        res += (n * 5isize.pow(i as u32)) as usize;
    }
    res
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}
#[aoc(day25, part1)]
pub fn part1(input: &[String]) -> String {
    to_snafu(input.iter().map(|s| from_snafu(s)).sum())
}
#[aoc(day25, part2)]
pub fn part2(_: &[String]) -> usize {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), "2=-1=0")
    }
}
