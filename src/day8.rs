use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| c - b'0').collect())
        .collect()
}

pub fn get_iter(
    i: usize,
    j: usize,
    h: usize,
    w: usize,
) -> Vec<(Box<dyn Iterator<Item = usize>>, bool)> {
    vec![
        (Box::new((0..i).rev()), false),
        (Box::new((i + 1)..h), false),
        (Box::new((0..j).rev()), true),
        (Box::new((j + 1)..w), true),
    ]
}

pub fn is_visible(m: &[Vec<u8>], i: usize, j: usize) -> bool {
    let h = m[i][j];
    get_iter(i, j, m.len(), m[0].len())
        .into_iter()
        .any(|(mut it, b)| it.all(|i1| !b && m[i1][j] < h || b && m[i][i1] < h))
}

pub fn score(m: &[Vec<u8>], i: usize, j: usize) -> usize {
    let h = m[i][j];
    get_iter(i, j, m.len(), m[0].len())
        .into_iter()
        .fold(1, |acc, (mut it, b)| {
            acc * it
                .fold_while(0, |acc2, i1| {
                    if !b && m[i1][j] < h || b && m[i][i1] < h {
                        Continue(acc2 + 1)
                    } else {
                        Done(acc2 + 1)
                    }
                })
                .into_inner()
        })
}

#[aoc(day8, part1)]
pub fn part1(input: &[Vec<u8>]) -> usize {
    (0..input.len())
        .cartesian_product(0..input[0].len())
        .fold(0, |acc, (i, j)| acc + is_visible(input, i, j) as usize)
}
#[aoc(day8, part2)]
pub fn part2(input: &[Vec<u8>]) -> usize {
    (0..input.len())
        .cartesian_product(0..input[0].len())
        .map(|(i, j)| score(input, i, j))
        .max()
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "30373
25512
65332
33549
35390
";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 21)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 8)
    }
}

// Original (and faster) solution
// pub fn is_visible(m: &[Vec<u8>], i: usize, j: usize) -> bool {
//     if i == 0 || j == 0 || i == m.len() - 1 || j == m[0].len() - 1 {
//         return true;
//     }
//     let h = m[i][j];
//     for i1 in (0..i).rev() {
//         if m[i1][j] >= h {
//             visible = false;
//             break;
//         }
//     }
//     if visible == true {
//         return true;
//     }
//     visible = true;
//
//     for i1 in (i + 1)..m.len() {
//         if m[i1][j] >= h {
//             visible = false;
//             break;
//         }
//     }
//     if visible == true {
//         return true;
//     }
//     visible = true;
//
//     for j1 in (0..j).rev() {
//         if m[i][j1] >= h {
//             visible = false;
//             break;
//         }
//     }
//     if visible == true {
//         return true;
//     }
//     visible = true;
//
//     for j1 in (j + 1)..m[0].len() {
//         if m[i][j1] >= h {
//             visible = false;
//             break;
//         }
//     }
//     visible
// }

// pub fn score(m: &[Vec<u8>], i: usize, j: usize) -> usize {
//     if i == 0 || j == 0 || i == m.len() - 1 || j == m[0].len() - 1 {
//         return 0;
//     }
//     let h = m[i][j];
//
//     let mut a = i;
//     for i1 in (0..i).rev() {
//         if m[i1][j] >= h {
//             a = i - i1;
//             break;
//         }
//     }
//     let mut b = m.len() - i - 1;
//     for i1 in (i + 1)..m.len() {
//         if m[i1][j] >= h || i1 == m.len() - 1 {
//             b = i1 - i;
//             break;
//         }
//     }
//     let mut c = j;
//     for j1 in (0..j).rev() {
//         if m[i][j1] >= h || j1 == 0 {
//             c = j - j1;
//             break;
//         }
//     }
//     let mut d = m[0].len() - j - 1;
//     for j1 in (j + 1)..m[0].len() {
//         if m[i][j1] >= h || j1 == m[0].len() - 1 {
//             d = j1 - j;
//             break;
//         }
//     }
//
//     a * b * c * d
// }
