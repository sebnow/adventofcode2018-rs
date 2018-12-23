extern crate aoc_runner;
extern crate rayon;
extern crate regex;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate aoc_runner_derive;

#[macro_use]
extern crate failure;

pub mod point;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day23;

aoc_lib! { year = 2018 }
