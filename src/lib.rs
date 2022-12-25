#![warn(clippy::all)]
#![feature(drain_filter)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(is_some_and)]

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
#[macro_use]
extern crate scan_fmt;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day23;
pub mod day24;
pub mod day25;
aoc_lib! { year = 2022 }
