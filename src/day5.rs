#[derive(Debug, Clone)]
pub struct Game {
    crates: Vec<Vec<char>>,
    inst: Vec<(usize, usize, usize)>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Game {
    let (stacks, insts) = input.split_once("\n\n").unwrap();
    let n = (stacks.lines().take(1).next().unwrap().len() + 1) / 4;
    let mut crates = vec![Vec::new(); n];
    stacks.lines().rev().skip(1).for_each(|l| {
        let mut i = 0;
        l.chars().collect::<Vec<char>>().chunks(4).for_each(|c| {
            if c[1] != ' ' {
                crates[i].push(c[1]);
            }
            i += 1;
        });
    });

    let inst = insts
        .lines()
        .map(|l| scan_fmt!(l, "move {d} from {d} to {d}", usize, usize, usize).unwrap())
        .collect::<Vec<_>>();
    Game { crates, inst }
}

impl Game {
    pub fn run_p1(&mut self) -> String {
        self.inst.iter().for_each(|inst| {
            for _ in 0..inst.0 {
                let tmp = self.crates[inst.1 - 1].pop().unwrap();
                self.crates[inst.2 - 1].push(tmp);
            }
        });
        self.crates.iter().map(|c| c.last().unwrap()).collect()
    }
    pub fn run_p2(&mut self) -> String {
        self.inst.iter().for_each(|inst| {
            let mut group = vec![' '; inst.0];
            for i in 0..inst.0 {
                let tmp = self.crates[inst.1 - 1].pop().unwrap();
                group[i] = tmp;
            }
            group.reverse();
            self.crates[inst.2 - 1].append(&mut group);
        });
        self.crates.iter().map(|c| c.last().unwrap()).collect()
    }
}
#[aoc(day5, part1)]
pub fn part1(input: &Game) -> String {
    input.clone().run_p1()
}

#[aoc(day5, part2)]
pub fn part2(input: &Game) -> String {
    input.clone().run_p2()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            )),
            "CMZ"
        )
    }
    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            )),
            "MCD"
        )
    }
}
