pub enum Cmd {
    Noop,
    Addx(isize),
}
use Cmd::*;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Cmd> {
    input
        .lines()
        .map(|l| match l.split_once(" ").unwrap_or((l, "")) {
            ("noop", _) => Noop,
            ("addx", i) => Addx(i.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect()
}

pub fn run(input: &[Cmd], cycles: &[usize]) -> isize {
    let mut reg = 1;
    let mut cycle = 1;
    input.iter().fold(0, |acc, c| {
        let mut ret = acc;
        match c {
            Noop => cycle += 1,
            Addx(x) => {
                if cycles.contains(&(cycle + 1)) {
                    ret = acc + reg * (cycle + 1) as isize;
                }
                reg += x;
                cycle += 2;
            }
        }
        if cycles.contains(&(cycle)) {
            ret = acc + reg * cycle as isize
        }
        ret
    })
}
pub fn run2(input: &[Cmd]) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; 40]; 6];
    let mut reg = 1 as isize;
    let mut cycle = 0 as usize;
    input.iter().for_each(|c| {
        if (reg as isize - (cycle % 40) as isize).abs() < 2 {
            grid[cycle / 40][cycle % 40] = '#';
        }

        match c {
            Noop => cycle += 1,
            Addx(x) => {
                if (reg as isize - ((cycle + 1) % 40) as isize).abs() < 2 {
                    grid[(cycle + 1) / 40][(cycle + 1) % 40] = '#';
                }
                reg += x;
                cycle += 2;
            }
        }
    });
    grid
}
#[aoc(day10, part1)]
pub fn part1(input: &[Cmd]) -> isize {
    run(input, &vec![20, 60, 100, 140, 180, 220])
}
#[aoc(day10, part2)]
pub fn part2(input: &[Cmd]) -> String {
    let g = run2(input);
    let mut res = "\n".to_owned();
    for j in g.iter() {
        for i in j.iter() {
            res.push(*i);
        }
        res.push('\n');
    }
    res
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 13140)
    }
    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(INPUT)),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        )
    }
}
