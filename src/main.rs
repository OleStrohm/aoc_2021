#![allow(dead_code)]
#![allow(unused)]

use std::slice::SliceIndex;

use itertools::Itertools;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    day19::day19();

    use std::collections::HashMap;

    let conversions = (0..2)
        .map(|_| (0, ((0, 0, 0), 0)))
        .map(|c| c)
        .collect::<HashMap<_, _>>();
}
