#![allow(dead_code)]
#![allow(unused)]

mod previous_days;

use itertools::Itertools;
use std::collections::HashMap;

fn step(state: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let new = state.get(0).map_or(0, |&(_, k)| k) + state.get(2).map_or(0, |&(a, _)| a);
    state
        .split_at(1)
        .1
        .iter()
        .copied()
        .chain(std::iter::once((new, new)))
        .collect()
}

fn day6() {
    let s = include_str!("day6.txt");
    let numbers = s
        .trim_end()
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .sorted()
        .group_by(|&n| n)
        .into_iter()
        .map(|(key, group)| (key, group.count() as _))
        .collect::<HashMap<_, _>>();
    let state = itertools::repeat_n((0, 0), 2)
        .chain((0..7).map(|d| (numbers.get(&d).copied().unwrap_or(0), 0)))
        .collect_vec();

    let part1 = (0..80).fold(state.clone(), |state, _| step(&state));

    println!(
        "part 1: {}",
        part1.iter().skip(2).map(|(a, _)| a).sum::<u64>()
            + part1.iter().map(|(_, k)| k).sum::<u64>()
    );

    let part2 = (0..256).fold(state, |state, _| step(&state));

    println!(
        "part 1: {}",
        part2.iter().skip(2).map(|(a, _)| a).sum::<u64>()
            + part2.iter().map(|(_, k)| k).sum::<u64>()
    );
}

fn main() {
    day6();
}
