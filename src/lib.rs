#![warn(clippy::all)]
#![feature(drain_filter)]
#![feature(box_patterns)]
#![feature(box_syntax)]

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;

pub mod day1;
pub mod day2;
pub mod day3;
aoc_lib! { year = 2022 }
